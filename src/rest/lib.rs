use reqwest::{Client, header::{HeaderMap, ACCEPT}, Method};
use serde_json;
use log::trace;
use std::fmt::Debug;

use super::RestMarketDataMessage;

pub trait RestMarketDataRequest{
    fn get_request_url(&self) -> String;
    fn get_request_headers(&self) -> HeaderMap{
        let mut map = HeaderMap::new();
        map.insert(ACCEPT, "application/json".parse().unwrap());
        map
    }
    fn get_request_method(&self) -> Method{
        Method::GET
    }
    fn get_request_body(&self) -> Option<String>{
        None
    }
}

// TODO: Add rate limiting
pub struct RestMarketDataClient{
    pub client: Client
}

impl RestMarketDataClient {
    pub fn new() -> RestMarketDataClient{
        RestMarketDataClient{
            client: Client::new()
        }
    }
    pub async fn process<T>(&self, message_params: T) -> Option<RestMarketDataMessage>
    where
        T: RestMarketDataRequest + Debug,
    {
        let request_method = message_params.get_request_method();
        trace!("Running {:?} for {:?}", request_method, message_params);
        match request_method{
            Method::GET => {Some(self.get(message_params).await)}
            Method::POST => {Some(self.post(message_params).await)}
            _ => {None}
        }
    }

    pub async fn get<T>(&self, message_params: T) -> RestMarketDataMessage
    where
        T: RestMarketDataRequest + Debug,
    {
        let request_url = message_params.get_request_url();
        let headers = message_params.get_request_headers();
        trace!("For {:?}, using url: {:?}, headers: {:?}", message_params, request_url, headers);

        let response = self.client.get(request_url).headers(headers).send().await.unwrap().text().await.unwrap();
        trace!("Got response: {:?}", response);
        let message: RestMarketDataMessage = serde_json::from_str(&response).unwrap();
        message
    }
    pub async fn post<T>(&self, message_params: T) -> RestMarketDataMessage
    where
        T: RestMarketDataRequest + Debug,
    {
        let request_url = message_params.get_request_url();
        let headers = message_params.get_request_headers();
        let body = message_params.get_request_body();
        trace!("For {:?}, using url: {:?}, headers: {:?}, body: {:?}", message_params, request_url, headers, body);

        let response = match body{
            None => {self.client.post(request_url).headers(headers).send().await.unwrap().text().await.unwrap()}
            Some(v) => {self.client.post(request_url).headers(headers).body(v).send().await.unwrap().text().await.unwrap()}
        };
        trace!("Got response: {:?}", response);
        let message: RestMarketDataMessage = serde_json::from_str(&response).unwrap();
        message
    }
}

impl Default for RestMarketDataClient{
    fn default() -> Self {
        Self::new()
    }
}