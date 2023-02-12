use super::lib::RestMarketDataRequest;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Method,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogoutMessage {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogoutMessageRequestParams {
    pub endpoint: String,
    pub token: String,
}

impl RestMarketDataRequest for LogoutMessageRequestParams {
    fn get_request_url(&self) -> String {
        let endpoint = self.endpoint.clone();
        format!("https://{endpoint}/trade-api/v2/logout")
    }
    fn get_request_method(&self) -> Method {
        Method::POST
    }
    fn get_request_headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        let token = self.token.clone();
        map.insert(AUTHORIZATION, token.parse().unwrap());
        map
    }
}
