use serde::{Deserialize, Serialize};
use super::lib::RestMarketDataRequest;
use crate::serializers::{Timestamp, from_iso8601_to_timestamp_nanos};



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomStrike{
    additional_prop: String,
} // TODO: add to markets when Kalshi implements it (it's just a skeleton in the Kalshi API rn)

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Market{
    pub can_close_early: bool,
    pub category: String,
    #[serde(rename(deserialize = "close_time"), deserialize_with="from_iso8601_to_timestamp_nanos")]
    pub close_timestamp: Timestamp,
    pub event_ticker: String,
    #[serde(rename(deserialize = "expiration_time"), deserialize_with="from_iso8601_to_timestamp_nanos")]
    pub expiration_timestamp: Timestamp,
    pub expiration_value: String,
    pub last_price: i64,
    pub liquidity: i64,
    pub no_ask: i64,
    pub no_bid: i64,
    pub open_interest: i64,
    #[serde(rename(deserialize = "open_time"), deserialize_with="from_iso8601_to_timestamp_nanos")]
    pub open_timestamp: Timestamp,
    pub previous_price: i64,
    pub previous_yes_ask: i64,
    pub previous_yes_bid: i64,
    pub result: String,
    pub risk_limit_cents: i64,
    pub subtitle: String,
    pub ticker: String,
    pub volume: i64,
    pub volume_24h: i64,
    pub yes_ask: i64,
    pub yes_bid: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketsMessage{
    pub cursor: String,
    pub markets: Vec<Market>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketMessage{
    pub market: Market
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketsMessageRequestParams{
    pub endpoint: String,
    pub limit: i32,
    pub cursor: String,
    pub event_ticker: String,
    pub series_ticker: String,
    pub max_close_ts: i64,
    pub min_close_ts: i64,
    pub status: String,
    pub tickers: Vec<String>
}

impl RestMarketDataRequest for MarketsMessageRequestParams{
    fn get_request_url(&self) -> String{
        let endpoint = self.endpoint.clone();
        let limit = self.limit;
        let cursor = self.cursor.clone();
        let event_ticker = self.event_ticker.clone();
        let series_ticker = self.series_ticker.clone();
        let max_close_ts = self.max_close_ts;
        let min_close_ts = self.min_close_ts;
        let status = self.status.clone();
        let tickers = self.tickers.clone().join(",");
        format!("https://{endpoint}/trade-api/v2/markets?limit={limit}&cursor={cursor}&event_ticker={event_ticker}&series_ticker={series_ticker}&max_close_ts={max_close_ts}&min_close_ts={min_close_ts}&status={status}&tickers={tickers}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketMessageRequestParams{
    pub endpoint: String,
    pub ticker: String,
}

impl RestMarketDataRequest for MarketMessageRequestParams{
    fn get_request_url(&self) -> String{
        let endpoint = self.endpoint.clone();
        let ticker = self.ticker.clone();
        format!("https://{endpoint}/trade-api/v2/markets/{ticker}")
    }
}