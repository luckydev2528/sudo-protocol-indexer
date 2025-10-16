pub mod schema;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::raffle_games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RaffleModule {
    pub id: i32,
    pub module_address: String,
    pub fa_metadata_address: String,
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub logo_uri: Option<String>,
    pub metadata_uri: Option<String>,
    pub price_per_ticket: String,
    pub tokens_per_raffle: String,
    pub circulating_supply: String,
    pub raffle_frequency: i32,
    pub frequency_unit: String,
    pub start_timestamp: String,
    pub end_timestamp: String,
    pub creator_address: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Get all active raffle modules from the database
pub fn get_active_raffle_modules(conn: &mut PgConnection) -> QueryResult<Vec<RaffleModule>> {
    use schema::raffle_games::dsl::*;
    
    raffle_games
        .filter(is_active.eq(true))
        .select(RaffleModule::as_select())
        .load::<RaffleModule>(conn)
}

/// Get all active raffle module addresses (just the addresses)
pub fn get_active_module_addresses(conn: &mut PgConnection) -> QueryResult<Vec<String>> {
    use schema::raffle_games::dsl::*;
    
    raffle_games
        .filter(is_active.eq(true))
        .select(module_address)
        .load::<String>(conn)
}

