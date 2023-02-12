use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscribeParams {
    pub channels: Vec<String>,
    pub market_ticker: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnsubscribeParams {
    #[serde(rename(deserialize = "sids"))]
    pub subscription_ids: Vec<i16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSubscriptionParams {
    #[serde(rename(deserialize = "sids"))]
    pub subscription_ids: Vec<i16>,
    pub market_tickers: Vec<String>,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WebsocketCommandParams {
    #[serde(rename(serialize = "params"))]
    Subscribe(SubscribeParams),
    #[serde(rename(serialize = "params"))]
    Unsubscribe(UnsubscribeParams),
    #[serde(rename(serialize = "params"))]
    UpdateSubscription(UpdateSubscriptionParams),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebsocketCommand {
    #[serde(rename(deserialize = "id", serialize = "id"))]
    pub command_id: u32,
    #[serde(rename(deserialize = "cmd", serialize = "cmd"))]
    pub command: String,
    #[serde(flatten)]
    pub params: WebsocketCommandParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscribeErrorResponseData {
    pub code: u32,
    #[serde(rename(deserialize = "msg", serialize = "msg"))]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscribeErrorResponse {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub response_type: String,
    #[serde(rename(deserialize = "id", serialize = "id"))]
    pub command_id: u32,
    #[serde(rename(deserialize = "msg", serialize = "msg"))]
    pub message: SubscribeErrorResponseData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscribeSuccessfulResponseData {
    pub channel: String,
    #[serde(rename(deserialize = "sid", serialize = "sid"))]
    pub subscription_id: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscribeSuccessfulResponse {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub response_type: String,
    #[serde(rename(deserialize = "id", serialize = "id"))]
    pub command_id: u32,
    #[serde(rename(deserialize = "msg", serialize = "msg"))]
    pub message: SubscribeSuccessfulResponseData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnsubscribeSuccessfulResponse {
    #[serde(rename(deserialize = "sid", serialize = "sid"))]
    pub subscription_id: i16,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub response_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSubscriptionSuccessfulResponse {
    #[serde(rename(deserialize = "id", serialize = "id"))]
    pub command_id: u32,
    #[serde(rename(deserialize = "sid", serialize = "sid"))]
    pub subscription_id: i16,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub response_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum WebsocketCommandResponse {
    SubscribeError(SubscribeErrorResponse),
    SubscribeSuccessful(SubscribeSuccessfulResponse),
    UnsubscribeSuccessful(UnsubscribeSuccessfulResponse),
    UpdateSubscriptionSuccessful(UpdateSubscriptionSuccessfulResponse),
}
