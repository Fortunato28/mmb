use derive_getters::Dissolve;

use crate::core::exchanges::common::*;
use crate::core::order_book::order_book_data::OrderBookData;
use crate::core::DateTime;

/// Possible variants of OrderBookEvent
#[derive(Debug, Copy, Clone)]
pub enum EventType {
    /// Means full snapshot should be add to local snapshots
    Snapshot,
    /// Means that data should be applied to suitable existing snapshot
    Update,
}

/// Event to update local snapshot
#[derive(Debug, Dissolve, Clone)]
pub struct OrderBookEvent {
    id: u128,
    pub creation_time: DateTime,
    pub exchange_account_id: ExchangeAccountId,
    pub currency_pair: CurrencyPair,

    event_id: String,

    event_type: EventType,
    data: OrderBookData,
}

impl OrderBookEvent {
    pub fn new(
        creation_time: DateTime,
        exchange_account_id: ExchangeAccountId,
        currency_pair: CurrencyPair,
        event_id: String,
        event_type: EventType,
        data: OrderBookData,
    ) -> OrderBookEvent {
        OrderBookEvent {
            id: 0,
            creation_time,
            exchange_account_id,
            currency_pair,
            event_id,
            event_type,
            data,
        }
    }

    /// Update inner OrderBookData
    pub fn apply_data_update(&mut self, updates: Vec<OrderBookData>) {
        self.data.update(updates);
    }
}
