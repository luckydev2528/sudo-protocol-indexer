use crate::raffle_events_model::RaffleEventModel;
use crate::buy_events_model::BuyEventModel;
use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::transaction::TxnData,
    postgres::{
        basic_processor::process,
        utils::database::{execute_in_chunks, MAX_DIESEL_PARAM_SIZE},
    },
};
use diesel::{pg::Pg, query_builder::QueryFragment, PgConnection, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use field_count::FieldCount;
use rayon::prelude::*;
use tracing::{error, info, warn};
use std::time::Duration;
use tokio::time::interval;
use axum::{
    routing::{get, post},
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod raffle_events_model;
pub mod buy_events_model;
pub mod db;
#[path = "db/schema.rs"]
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

fn insert_raffle_events_query(
    items_to_insert: Vec<RaffleEventModel>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    use crate::schema::raffle_events::dsl::*;
    diesel::insert_into(crate::schema::raffle_events::table)
        .values(items_to_insert)
        .on_conflict((transaction_version, event_index))
        .do_nothing()
}

fn insert_buy_events_query(
    items_to_insert: Vec<BuyEventModel>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    use crate::schema::buy_events::dsl::*;
    diesel::insert_into(crate::schema::buy_events::table)
        .values(items_to_insert)
        .on_conflict((transaction_version, event_index))
        .do_nothing()
}

#[derive(Debug, Serialize, Deserialize)]
struct ReloadResponse {
    success: bool,
    message: String,
    modules_loaded: usize,
}

/// Load active modules from database and update the global registry
fn load_modules_from_db(database_url: &str) -> Result<Vec<String>> {
    let mut conn = PgConnection::establish(database_url)?;
    let module_addresses = db::get_active_module_addresses(&mut conn)?;
    
    // Update both event model registries
    buy_events_model::update_active_modules(module_addresses.clone());
    raffle_events_model::update_active_modules(module_addresses.clone());
    
    Ok(module_addresses)
}

/// HTTP handler for reload endpoint
async fn reload_modules_handler(
    axum::extract::State(database_url): axum::extract::State<Arc<String>>,
) -> impl IntoResponse {
    match load_modules_from_db(&database_url) {
        Ok(modules) => {
            let response = ReloadResponse {
                success: true,
                message: format!("Successfully reloaded {} active raffle modules", modules.len()),
                modules_loaded: modules.len(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            error!("Failed to reload modules: {:?}", e);
            let response = ReloadResponse {
                success: false,
                message: format!("Failed to reload modules: {}", e),
                modules_loaded: 0,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

/// HTTP handler for health check
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Indexer is running")
}

/// Start HTTP server for reload endpoint
async fn start_http_server(database_url: Arc<String>) {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/reload-modules", post(reload_modules_handler))
        .with_state(database_url);

    println!("🌐 Starting HTTP server on http://0.0.0.0:8086");
    println!("   - Health check: http://localhost:8086/health");
    println!("   - Reload endpoint: http://localhost:8086/reload-modules (POST)");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8086")
        .await
        .expect("Failed to bind HTTP server");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start HTTP server");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Note: tracing is initialized by the Aptos SDK's process() function
    
    println!("🚀 Starting SUDO Raffle Indexer with Dynamic Module Loading");
    
    // Get database URL
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/sudo_indexer".to_string());
    
    // Load initial modules from database
    println!("📋 Loading initial raffle modules from database...");
    match load_modules_from_db(&database_url) {
        Ok(modules) => {
            println!("✅ Initial load complete: {} active raffle modules", modules.len());
        }
        Err(e) => {
            warn!("⚠️ Failed to load initial modules: {}. Will retry periodically.", e);
        }
    }
    
    // Wrap database URL in Arc for sharing between tasks
    let shared_db_url = Arc::new(database_url.clone());
    
    // Spawn background task for periodic module refresh (every 60 seconds)
    let refresh_db_url = shared_db_url.clone();
    tokio::spawn(async move {
        let mut refresh_interval = interval(Duration::from_secs(60));
        
        loop {
            refresh_interval.tick().await;
            
            println!("🔄 [Periodic Refresh] Checking for new raffle modules...");
            match load_modules_from_db(&refresh_db_url) {
                Ok(modules) => {
                    println!("✅ [Periodic Refresh] Successfully refreshed modules: {} active", modules.len());
                }
                Err(e) => {
                    error!("❌ [Periodic Refresh] Failed to refresh modules: {:?}", e);
                }
            }
        }
    });
    
    // Spawn HTTP server for instant reload endpoint
    let http_db_url = shared_db_url.clone();
    tokio::spawn(async move {
        start_http_server(http_db_url).await;
    });
    
    println!("📡 Starting blockchain event processor...");
    
    // Start the main processor
    process(
        "events_processor".to_string(),
        MIGRATIONS,
        async |transactions, conn_pool| {
            // process raffle events
            let raffle_events = transactions
                .par_iter()
                .map(|txn| {
                    let txn_version = txn.version as i64;
                    let block_height = txn.block_height as i64;
                    let txn_data = match txn.txn_data.as_ref() {
                        Some(data) => data,
                        None => {
                            warn!(
                                transaction_version = txn_version,
                                "Transaction data doesn't exist"
                            );
                            return vec![];
                        },
                    };
                    let default = vec![];
                    let raw_events = match txn_data {
                        TxnData::BlockMetadata(tx_inner) => &tx_inner.events,
                        TxnData::Genesis(tx_inner) => &tx_inner.events,
                        TxnData::User(tx_inner) => &tx_inner.events,
                        _ => &default,
                    };

                    RaffleEventModel::from_events(raw_events, txn_version, block_height)
                })
                .flatten()
                .collect::<Vec<RaffleEventModel>>();

            // Store raffle events in the database
            let mut execute_res = execute_in_chunks(
                conn_pool.clone(),
                insert_raffle_events_query,
                &raffle_events,
                MAX_DIESEL_PARAM_SIZE / RaffleEventModel::field_count(),
            )
            .await;
            match execute_res {
                Ok(_) => {
                    if !raffle_events.is_empty() {
                        info!("✅ Stored {} raffle events", raffle_events.len());
                    }
                },
                Err(e) => {
                    error!("❌ Failed to store raffle events: {:?}", e);
                },
            };


            // process buy events
            let buy_events = transactions
                .par_iter()
                .map(|txn| {
                    let txn_version = txn.version as i64;
                    let block_height = txn.block_height as i64;
                    let txn_data = match txn.txn_data.as_ref() {
                        Some(data) => data,
                        None => {
                            warn!(
                                transaction_version = txn_version,
                                "Transaction data doesn't exist"
                            );
                            return vec![];
                        },
                    };
                    let default = vec![];
                    let raw_events = match txn_data {
                        TxnData::BlockMetadata(tx_inner) => &tx_inner.events,
                        TxnData::Genesis(tx_inner) => &tx_inner.events,
                        TxnData::User(tx_inner) => &tx_inner.events,
                        _ => &default,
                    };

                    BuyEventModel::from_events(raw_events, txn_version, block_height)
                })
                .flatten()
                .collect::<Vec<BuyEventModel>>();

            // Store buy events in the database
            execute_res = execute_in_chunks(
                conn_pool.clone(),
                insert_buy_events_query,
                &buy_events,
                MAX_DIESEL_PARAM_SIZE / BuyEventModel::field_count(),
            )
            .await;
            match execute_res {
                Ok(_) => {
                    if !buy_events.is_empty() {
                        info!("✅ Stored {} buy events", buy_events.len());
                    }
                    Ok(())
                },
                Err(e) => {
                    error!("❌ Failed to store buy events: {:?}", e);
                    Err(e)
                },
            }
        },
    )
    .await?;
    Ok(())
}
