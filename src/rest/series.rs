use super::lib::RestMarketDataRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeriesMessage {
    pub frequency: String,
    pub ticker: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeriesMessageRequestParams {
    pub endpoint: String,
    pub series_ticker: String,
}

impl RestMarketDataRequest for SeriesMessageRequestParams {
    fn get_request_url(&self) -> String {
        let endpoint = self.endpoint.clone();
        let series_ticker = self.series_ticker.clone();
        format!("https://{endpoint}/trade-api/v2/series/{series_ticker}")
    }
}
