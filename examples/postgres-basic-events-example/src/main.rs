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
use diesel::{pg::Pg, query_builder::QueryFragment};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use field_count::FieldCount;
use rayon::prelude::*;
use tracing::{error, info, warn};

pub mod raffle_events_model;
pub mod buy_events_model;
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

// fn update_buy_events_query(
//     items_to_update: Vec<RaffleEventModel>
// ) {
//     use crate::schema::buy_events::dsl::*;

//     for item in items_to_update {
//         diesel::update(
//             buy_events
//                 .filter(buy_events::transaction_version.eq(item.transaction_version))
//                 .filter(buy_events::sequence.eq(item.sequence))
//         )
//         .set(buy_events::status.eq(true))
//         .get_result();
//     }
// }

fn insert_buy_events_query(
    items_to_insert: Vec<BuyEventModel>,
) -> impl QueryFragment<Pg> + diesel::query_builder::QueryId + Send {
    use crate::schema::buy_events::dsl::*;
    diesel::insert_into(crate::schema::buy_events::table)
        .values(items_to_insert)
        .on_conflict((transaction_version, event_index))
        .do_nothing()
}

#[tokio::main]
async fn main() -> Result<()> {
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
                    // info!(
                    //     "Events version [{}, {}] stored successfully",
                    //     transactions.first().unwrap().version,
                    //     transactions.last().unwrap().version
                    // );
                    // Ok(())
                },
                Err(e) => {
                    error!("Failed to store events: {:?}", e);
                    // Err(e)
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
                    // info!(
                    //     "Events version [{}, {}] stored successfully",
                    //     transactions.first().unwrap().version,
                    //     transactions.last().unwrap().version
                    // );
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to store events: {:?}", e);
                    Err(e)
                },
            }
        },
    )
    .await?;
    Ok(())
}
