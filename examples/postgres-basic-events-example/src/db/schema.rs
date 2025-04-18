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
        // num_tickets -> Int8,
        amount_apt -> Int8,
        #[sql_name = "timestamp"]
        timestamp_ -> Int8,
        // status -> Bool,
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
        #[sql_name = "timestamp"]
        timestamp_ -> Int8,
        inserted_at -> Timestamp,
        event_index -> Int8,
        #[max_length = 300]
        indexed_type -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    buy_events,
    raffle_events,
);
