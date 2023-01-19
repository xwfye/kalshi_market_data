use serde::{Deserialize, Serialize};
use crate::serializers::{
    from_timestamp_seconds_to_timestamp_nanos,
    Timestamp
};


use super::lib::MessageArbitration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickerData {
    pub market_id: String,
    pub price: i64,
    pub yes_bid: i64,
    pub yes_ask: i64,
    pub volume: i64,
    pub open_interest: i64,
    pub dollar_volume: i64,
    pub dollar_open_interest: i64,
    #[serde(rename(deserialize = "ts"), deserialize_with="from_timestamp_seconds_to_timestamp_nanos")]
    pub timestamp: Timestamp
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickerMessage {
    #[serde(rename(deserialize = "type"))]
    pub stream_type: String,
    #[serde(rename(deserialize = "sid"))]
    pub subscription_id: i16,
    #[serde(rename(deserialize = "msg"), flatten)]
    pub data: TickerData
}

impl MessageArbitration for TickerMessage{
    fn get_sequence_num(&self) -> u64 {
        self.data.timestamp
    }
}