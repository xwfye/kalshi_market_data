use serde::{Deserialize, Serialize};
use super::lib::RestMarketDataRequest;
use crate::serializers::{Timestamp, from_timestamp_millis_to_timestamp_nanos};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History{
    pub no_ask: i64,
    pub no_bid: i64,
    pub open_interest: i64,
    #[serde(rename(deserialize = "ts"), deserialize_with="from_timestamp_millis_to_timestamp_nanos")]
    pub timestamp: Timestamp,
    pub volume: i64,
    pub yes_ask: i64,
    pub yes_bid: i64,
    pub yes_price: i64
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryMessage{
    pub cursor: String,
    pub history: Vec<History>,
    pub ticker: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryMessageRequestParams{
    pub endpoint: String,
    pub ticker: String,
    pub limit: i32,
    pub cursor: String,
    pub min_ts: i64,
    pub max_ts: i64,
}

impl RestMarketDataRequest for HistoryMessageRequestParams{
    fn get_request_url(&self) -> String{
        let endpoint = self.endpoint.clone();
        let ticker = self.ticker.clone();
        let limit = self.limit;
        let cursor = self.cursor.clone();
        let min_ts = self.min_ts;
        let max_ts = self.max_ts;
        format!("https://{endpoint}/trade-api/v2/markets/{ticker}/history?limit={limit}&cursor={cursor}&min_ts={min_ts}&max_ts={max_ts}")
    }
}