// @generated automatically by Diesel CLI.

diesel::table! {
    buy_events (transaction_version, event_index) {
        sequence_number -> Int8,
        creation_number -> Int8,
        #[max_length = 66]
        account_address -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        coin_type -> Text,
        sequence -> Int8,
        buyer -> Text,
        amount_apt -> Int8,
        timestamp -> Int8,
        inserted_at -> Timestamp,
        event_index -> Int8,
        #[max_length = 300]
        indexed_type -> Varchar,
    }
}

diesel::table! {
    raffle_events (transaction_version, event_index) {
        sequence_number -> Int8,
        creation_number -> Int8,
        #[max_length = 66]
        account_address -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        coin_type -> Text,
        sequence -> Int8,
        winner -> Text,
        total_tickets -> Int8,
        amount_apt -> Int8,
        amount_token -> Int8,
        timestamp -> Int8,
        inserted_at -> Timestamp,
        event_index -> Int8,
        #[max_length = 300]
        indexed_type -> Varchar,
    }
}

diesel::table! {
    raffle_games (id) {
        id -> Int4,
        #[max_length = 255]
        module_address -> Varchar,
        #[max_length = 255]
        fa_metadata_address -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 500]
        logo_uri -> Nullable<Varchar>,
        #[max_length = 500]
        metadata_uri -> Nullable<Varchar>,
        #[max_length = 100]
        price_per_ticket -> Varchar,
        #[max_length = 100]
        tokens_per_raffle -> Varchar,
        #[max_length = 100]
        circulating_supply -> Varchar,
        raffle_frequency -> Int4,
        #[max_length = 20]
        frequency_unit -> Varchar,
        #[max_length = 100]
        start_timestamp -> Varchar,
        #[max_length = 100]
        end_timestamp -> Varchar,
        #[max_length = 255]
        creator_address -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(buy_events, raffle_events, raffle_games,);
