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


// p99 currently is 303 so using 300 as a safe max length
const EVENT_TYPE_MAX_LENGTH: usize = 300;


#[derive(Clone, Debug, Deserialize, Serialize)]
/// On-chain representation of a message creation event
pub struct RaffleEventOnChain {
    pub coin_type: String,
    pub sequence: String,
    pub winner: String,
    pub total_tickets: String,
    pub amount_apt: String,
    pub amount_token: String,
    pub timestamp: String
}


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
    pub coin_type: String,
    pub sequence: i64,
    pub winner: String,
    pub total_tickets: i64,
    pub amount_apt: i64,
    pub amount_token: i64,
    pub timestamp_: i64,
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

        // Monitor events from the new deployed module
        if t.starts_with("0xb359560fc77127a3d1ccdad4bd7f8423997a16a3572dcea4e4dd747fd3ecd08b::meme::RaffleEvent") ||
           t.starts_with("0xc58654a8eaa2818496ab82ae67dbae67963cd429267da4db1039de0bb712ef07::meme::RaffleEvent") || 
           t.starts_with("0xd7726cb41ba5b84c22af0038066baab6cf892ef9191cce0a6137f4642d57be5e::meme::RaffleEvent") {
            let data: RaffleEventOnChain = serde_json::from_str(event.data.as_str()).unwrap();
            // info!("");
            // info!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            // info!("raffle_event: {:?}", data);
            // info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            // info!("");

            Some(RaffleEvent {
                account_address: standardize_address(
                    event.key.as_ref().unwrap().account_address.as_str(),
                ),
                creation_number: event.key.as_ref().unwrap().creation_number as i64,
                sequence_number: event.sequence_number as i64,
                transaction_version,
                transaction_block_height,
                type_: t.to_string(),
                coin_type: data.coin_type,
                sequence: data.sequence.parse().unwrap(),
                winner: data.winner,
                total_tickets: data.total_tickets.parse().unwrap(),
                amount_apt: data.amount_apt.parse().unwrap(),
                amount_token: data.amount_token.parse().unwrap(),
                timestamp_: data.timestamp.parse().unwrap(),
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
            .collect::<Vec<RaffleEvent>>()
    }
}

// Prevent conflicts with other things named `Event`
pub type RaffleEventModel = RaffleEvent;
