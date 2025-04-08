// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::extra_unused_lifetimes)]

use crate::schema::raffle_events;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::Event as EventPB,
    utils::convert::{standardize_address, truncate_str},
};
use diesel::{Identifiable, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};
use tracing::{info};


#[derive(Clone, Debug, Deserialize, Serialize)]
/// On-chain representation of a message creation event
pub struct RaffleEventOnChain {
    pub winner: String,
    pub coin_type: String,
    pub timestamp: String
}


// p99 currently is 303 so using 300 as a safe max length
const EVENT_TYPE_MAX_LENGTH: usize = 300;

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, event_index))]
#[diesel(table_name = raffle_events)]
pub struct RaffleEvent {
    pub sequence_number: i64,
    pub creation_number: i64,
    pub account_address: String,
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub type_: String,
    // pub data: serde_json::Value,
    pub winner: String,
    pub coin_type: String,
    pub timestamp_: String,
    pub event_index: i64,
    pub indexed_type: String,
}

impl RaffleEvent {
    pub fn from_event(
        event: &EventPB,
        transaction_version: i64,
        transaction_block_height: i64,
        event_index: i64,
    ) -> Option<Self> {
        let t: &str = event.type_str.as_ref();

        if t.starts_with("0x48db28693cf47be4fb9a37c51d1e6cb10c1301b72955c71d31675e3daa549da9::meme::RaffleEvent") {
            let data: RaffleEventOnChain = serde_json::from_str(event.data.as_str()).unwrap();
            info!("");
            info!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            info!("raffle_event: {:?}", data);
            info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            info!("");

            Some(RaffleEvent {
                account_address: standardize_address(
                    event.key.as_ref().unwrap().account_address.as_str(),
                ),
                creation_number: event.key.as_ref().unwrap().creation_number as i64,
                sequence_number: event.sequence_number as i64,
                transaction_version,
                transaction_block_height,
                type_: t.to_string(),
                // data: serde_json::from_str(event.data.as_str()).unwrap(),
                winner: data.winner,
                coin_type: data.coin_type,
                timestamp_: data.timestamp,
                event_index,
                indexed_type: truncate_str(t, EVENT_TYPE_MAX_LENGTH),
            })
        } else {
            None
        }
    }

    pub fn from_events(
        events: &[EventPB],
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Vec<Self> {
        events
            .iter()
            .enumerate()
            .filter_map(|(index, event)| {
                Self::from_event(
                    event,
                    transaction_version,
                    transaction_block_height,
                    index as i64,
                )
            })
            .collect::<Vec<RaffleEventModel>>()
    }
}

// Prevent conflicts with other things named `Event`
pub type RaffleEventModel = RaffleEvent;
