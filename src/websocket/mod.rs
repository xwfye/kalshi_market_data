use serde::{Deserialize, Serialize};

use self::{
    fill::FillMessage,
    lib::MessageArbitration,
    orderbook::{OrderbookDeltaMessage, OrderbookSnapshotMessage},
    ticker::TickerMessage,
    trade::TradeMessage,
};

pub mod commands;
pub mod fill;
pub mod lib;
pub mod orderbook;
pub mod ticker;
pub mod trade;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WebsocketMarketDataMessage {
    Ticker(TickerMessage),
    OrderbookSnapshot(OrderbookSnapshotMessage),
    OrderbookDelta(OrderbookDeltaMessage),
    Trade(TradeMessage),
    Fill(FillMessage),
}

impl MessageArbitration for WebsocketMarketDataMessage {
    fn get_sequence_num(&self) -> u64 {
        match self {
            WebsocketMarketDataMessage::Ticker(v) => v.get_sequence_num(),
            WebsocketMarketDataMessage::OrderbookSnapshot(v) => v.get_sequence_num(),
            WebsocketMarketDataMessage::OrderbookDelta(v) => v.get_sequence_num(),
            WebsocketMarketDataMessage::Trade(v) => v.get_sequence_num(),
            WebsocketMarketDataMessage::Fill(v) => v.get_sequence_num(),
        }
    }
}
