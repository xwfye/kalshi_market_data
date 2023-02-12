use futures_util::{SinkExt, StreamExt};

use super::commands::{
    SubscribeParams, WebsocketCommand, WebsocketCommandParams, WebsocketCommandResponse,
};
use super::WebsocketMarketDataMessage;
use tungstenite::client::IntoClientRequest;
use tungstenite::error::{Error, UrlError};
use tungstenite::handshake::client::{generate_key, Request};
use tungstenite::http::Uri;
use tungstenite::Message;

use flume::Sender;
use tokio_tungstenite::connect_async;
pub trait MessageArbitration {
    fn get_sequence_num(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct MarketDataConfig {
    pub endpoint: String,
    pub subscription: SubscribeParams,
}

#[derive(Debug, Clone)]
pub struct WebsocketMarketDataRequest {
    pub endpoint: String,
    pub token: String,
}

impl IntoClientRequest for WebsocketMarketDataRequest {
    fn into_client_request(self) -> tungstenite::error::Result<Request> {
        let endpoint_uri = self.endpoint.parse::<Uri>()?;
        let authority = endpoint_uri
            .authority()
            .ok_or(Error::Url(UrlError::NoHostName))?
            .as_str();
        let host = authority
            .find('@')
            .map(|idx| authority.split_at(idx + 1).1)
            .unwrap_or_else(|| authority);

        if host.is_empty() {
            return Err(Error::Url(UrlError::EmptyHostName));
        }

        let req = Request::builder()
            .method("GET")
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .header("Authorization", self.token.as_str())
            .uri(endpoint_uri)
            .body(())?;
        Ok(req)
    }
}

impl Unpin for WebsocketMarketDataRequest {}

// TODO add multiple connections and arbitration
#[derive(Debug, Clone)]
pub struct WebsocketMarketDataClient {
    pub config: MarketDataConfig,
    pub token: String,
}

impl WebsocketMarketDataClient {
    pub async fn start(&self, message_sink: Sender<WebsocketMarketDataMessage>) {
        let ws_request = WebsocketMarketDataRequest {
            endpoint: self.config.endpoint.clone(),
            token: self.token.clone(),
        };

        let (mut websocket_connection, _) = connect_async(ws_request.clone())
            .await
            .expect("Failed to connect");

        let websocket_command = WebsocketCommand {
            command_id: 1,
            command: "subscribe".to_string(),
            params: WebsocketCommandParams::Subscribe(self.config.subscription.clone()),
        };

        websocket_connection
            .send(Message::Text(
                serde_json::to_string(&websocket_command).unwrap(),
            ))
            .await
            .unwrap();

        let command_response: WebsocketCommandResponse = serde_json::from_str(
            &websocket_connection
                .next()
                .await
                .unwrap()
                .unwrap()
                .into_text()
                .unwrap(),
        )
        .unwrap();

        match command_response {
            WebsocketCommandResponse::SubscribeError(v) => {
                println!("{:?}", v);
            } // TODO: add proper handling
            WebsocketCommandResponse::SubscribeSuccessful(v) => {
                println!("{:?}", v);
            }
            _ => {}
        }

        while let Some(message) = websocket_connection.next().await {
            let message_unwrapped = message.unwrap().into_text().unwrap();
            if let Ok(v) = serde_json::from_str(&message_unwrapped) {
                message_sink.send(v).unwrap();
            }
        }
    }
}
