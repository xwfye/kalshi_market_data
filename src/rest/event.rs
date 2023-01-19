use serde::{Deserialize, Serialize};

use super::market::Market;
use super::lib::RestMarketDataRequest;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventData{
    pub category: String,
    pub event_ticker: String,
    pub mutually_exclusive: bool,
    pub series_ticker: String,
    #[serde(rename(deserialize = "sub_title"))]
    pub subtitle: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventMessage{
    pub event: EventData,
    pub markets: Vec<Market>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventMessageRequestParams{
    pub endpoint: String,
    pub event_ticker: String,
}

impl RestMarketDataRequest for EventMessageRequestParams{
    fn get_request_url(&self) -> String{
        let endpoint = self.endpoint.clone();
        let event_ticker = self.event_ticker.clone();
        format!("https://{endpoint}/trade-api/v2/events/{event_ticker}")
    }
}