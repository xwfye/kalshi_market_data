use super::lib::RestMarketDataRequest;
use crate::serializers::{from_iso8601_to_timestamp_nanos, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    pub count: i64,
    #[serde(
        rename(deserialize = "created_time"),
        deserialize_with = "from_iso8601_to_timestamp_nanos"
    )]
    pub created_timestamp: Timestamp,
    pub no_price: i64,
    pub taker_side: String,
    pub ticker: String,
    pub trade_id: String,
    pub yes_price: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradesMessage {
    pub cursor: String,
    pub markets: Vec<Trade>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeMessageRequestParams {
    pub endpoint: String,
    pub cursor: String,
    pub limit: i32,
    pub ticker: String,
    pub min_ts: i64,
    pub max_ts: i64,
}

impl RestMarketDataRequest for TradeMessageRequestParams {
    fn get_request_url(&self) -> String {
        let endpoint = self.endpoint.clone();
        let cursor = self.cursor.clone();
        let limit = self.limit;
        let ticker = self.ticker.clone();
        let min_ts = self.min_ts;
        let max_ts = self.max_ts;
        format!("https://{endpoint}/trade-api/v2/markets/trades?cursor={cursor}&limit={limit}&ticker={ticker}&min_ts={min_ts}&max_ts={max_ts}")
    }
}
