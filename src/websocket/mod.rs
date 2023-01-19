use serde::{Serialize, Deserialize};


use self::{ticker::TickerMessage, orderbook::{OrderbookSnapshotMessage, OrderbookDeltaMessage}, trade::TradeMessage, fill::FillMessage, lib::MessageArbitration};

pub mod orderbook;
pub mod ticker;
pub mod commands;
pub mod trade;
pub mod fill;
pub mod lib;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WebsocketMarketDataMessage {
    Ticker(TickerMessage),
    OrderbookSnapshot(OrderbookSnapshotMessage),
    OrderbookDelta(OrderbookDeltaMessage),
    Trade(TradeMessage),
    Fill(FillMessage)
}


impl MessageArbitration for WebsocketMarketDataMessage{
    fn get_sequence_num(&self) -> u64 {
        match self {
            WebsocketMarketDataMessage::Ticker(v) => {
                v.get_sequence_num()
            }
            WebsocketMarketDataMessage::OrderbookSnapshot(v) => {
                v.get_sequence_num()
            }
            WebsocketMarketDataMessage::OrderbookDelta(v) => {
                v.get_sequence_num()
            }
            WebsocketMarketDataMessage::Trade(v) => {
                v.get_sequence_num()
            }
            WebsocketMarketDataMessage::Fill(v) => {
                v.get_sequence_num()
            }
        }
    }
}