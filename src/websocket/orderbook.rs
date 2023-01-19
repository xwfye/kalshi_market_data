use serde::{Deserialize, Serialize};

use super::lib::MessageArbitration;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookSnapshotData {
    pub market_id: String,
    pub yes: Vec<Vec<i64>>,
    pub no: Vec<Vec<i64>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookSnapshotMessage {
    #[serde(rename(deserialize = "type"))]
    pub stream_type: String,
    #[serde(rename(deserialize = "sid"))]
    pub subscription_id: i64,
    #[serde(rename(deserialize = "seq"))]
    pub sequence_num: u64,
    #[serde(rename(deserialize = "msg"), flatten)]
    pub data: OrderbookSnapshotData
}


impl MessageArbitration for OrderbookSnapshotMessage{
    fn get_sequence_num(&self) -> u64 {
        self.sequence_num
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookDeltaData {
    pub market_id: String,
    pub price: i64,
    pub delta: i64,
    pub side: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookDeltaMessage {
    #[serde(rename(deserialize = "type"))]
    pub stream_type: String,
    #[serde(rename(deserialize = "sid"))]
    pub subscription_id: i64,
    #[serde(rename(deserialize = "seq"))]
    pub sequence_num: u64,
    #[serde(rename(deserialize = "msg"), flatten)]
    pub data: OrderbookDeltaData
}


impl MessageArbitration for OrderbookDeltaMessage{
    fn get_sequence_num(&self) -> u64 {
        self.sequence_num
    }
}