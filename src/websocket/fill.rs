use crate::serializers::{from_timestamp_seconds_to_timestamp_nanos, Timestamp};
use serde::{Deserialize, Serialize};

use super::lib::MessageArbitration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FillData {
    pub trade_id: String,
    pub order_id: String,
    pub market_ticker: String,
    pub is_taker: bool,
    pub side: String,
    pub yes_price: i64,
    pub no_price: i64,
    pub count: i64,
    pub action: String,
    #[serde(
        rename(deserialize = "ts"),
        deserialize_with = "from_timestamp_seconds_to_timestamp_nanos"
    )]
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FillMessage {
    #[serde(rename(deserialize = "type"))]
    pub stream_type: String,
    #[serde(rename(deserialize = "sid"))]
    pub subscription_id: i16,
    #[serde(rename(deserialize = "msg"), flatten)]
    pub data: FillData,
}

impl MessageArbitration for FillMessage {
    fn get_sequence_num(&self) -> u64 {
        self.data.timestamp
    }
}
