use serde::{Deserialize, Serialize};
use crate::serializers::{
    from_timestamp_seconds_to_timestamp_nanos,
    Timestamp
};


use super::lib::MessageArbitration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeData {
    pub market_ticker: String,
    pub yes_price: i64,
    pub no_price: i64,
    pub count: i64,
    pub taker_side: String,
    #[serde(rename(deserialize = "ts"), deserialize_with="from_timestamp_seconds_to_timestamp_nanos")]
    pub timestamp: Timestamp
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeMessage {
    #[serde(rename(deserialize = "type"))]
    pub stream_type: String,
    #[serde(rename(deserialize = "sid"))]
    pub subscription_id: i16,
    #[serde(rename(deserialize = "msg"), flatten)]
    pub data: TradeData
}

impl MessageArbitration for TradeMessage{
    fn get_sequence_num(&self) -> u64 {
        self.data.timestamp
    }
}