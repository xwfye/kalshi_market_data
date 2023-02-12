use super::lib::RestMarketDataRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeStatusMessage {
    pub exchange_active: bool,
    pub trading_active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeStatusMessageRequestParams {
    pub endpoint: String,
}

impl RestMarketDataRequest for ExchangeStatusMessageRequestParams {
    fn get_request_url(&self) -> String {
        let endpoint = self.endpoint.clone();
        format!("https://{endpoint}/trade-api/v2/exchange/status")
    }
}
