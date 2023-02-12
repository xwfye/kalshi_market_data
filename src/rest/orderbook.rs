use super::lib::RestMarketDataRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Orderbook {
    pub yes: Vec<Vec<i64>>, // TODO: Add custom deserializers to convert Vec of Vec to a simple ordered map? Might be cleaner do here
    pub no: Vec<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookMessage {
    pub orderbook: Orderbook,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookMessageRequestParams {
    pub endpoint: String,
    pub ticker: String,
    pub depth: i32,
}

impl RestMarketDataRequest for OrderbookMessageRequestParams {
    fn get_request_url(&self) -> String {
        let endpoint = self.endpoint.clone();
        let ticker = self.ticker.clone();
        let depth = self.depth;
        format!("https://{endpoint}/trade-api/v2/markets/{ticker}/orderbook?depth={depth}")
    }
}
