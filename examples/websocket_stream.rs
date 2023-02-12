use kalshi_market_data::{
    rest::{
        lib::RestMarketDataClient,
        login::{LoginMessage, LoginMessageRequestParams},
        logout::LogoutMessageRequestParams,
        RestMarketDataMessage,
    },
    websocket::{
        commands::SubscribeParams,
        lib::{MarketDataConfig, WebsocketMarketDataClient},
    },
};
use serde::{Deserialize};
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
struct RunConfig {
    pub market_ticker: String,
    pub channel: String,
    pub rest_endpoint: String,
    pub websocket_endpoint: String,
}

#[tokio::main]
pub async fn main() {
    let path = std::env::args().nth(1).expect("No config path given");

    let config: RunConfig =
        toml::from_str(&fs::read_to_string(&path).expect("Path for config not valid"))
            .expect("Config provided is not parsable or incomplete");

    let email = env::var("KALSHI_EMAIL").unwrap();
    let password = env::var("KALSHI_PASSWORD").unwrap();

    let subscription_params = SubscribeParams {
        channels: vec![config.channel],
        market_ticker: config.market_ticker,
    };
    let market_data_config = MarketDataConfig {
        endpoint: config.websocket_endpoint,
        subscription: subscription_params,
    };

    let login_request = LoginMessageRequestParams {
        endpoint: config.rest_endpoint,
        email: email,
        password: password,
    };

    let rest_client = RestMarketDataClient::new();
    let login_rsp: LoginMessage = match rest_client.process(login_request).await.unwrap() {
        RestMarketDataMessage::Login(v) => Some(v),
        _ => None,
    }
    .unwrap();
    println!("{}", serde_json::to_string(&login_rsp).unwrap());

    let ws_client = WebsocketMarketDataClient {
        config: market_data_config.clone(),
        token: login_rsp.token.clone(),
    };

    let (sender, receiver) = flume::unbounded();

    tokio::spawn(async move {
        ws_client.start(sender).await;
    });

    for message in receiver.iter() {
        println!("{:?}", message);
    }

    let logout_request = LogoutMessageRequestParams {
        endpoint: "trading-api.kalshi.com".to_string(),
        token: login_rsp.token.clone(),
    };

    let _ = rest_client.process(logout_request).await.unwrap();
}
