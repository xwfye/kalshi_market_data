pub mod error;
pub mod event;
pub mod exchange;
pub mod history;
pub mod lib;
pub mod login;
pub mod logout;
pub mod market;
pub mod orderbook;
pub mod series;
pub mod trade;

use serde::{Deserialize, Serialize};

use self::{
    error::ErrorMessage,
    event::EventMessage,
    exchange::ExchangeStatusMessage,
    history::HistoryMessage,
    login::LoginMessage,
    logout::LogoutMessage,
    market::{MarketMessage, MarketsMessage},
    orderbook::OrderbookMessage,
    series::SeriesMessage,
    trade::TradesMessage,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RestMarketDataMessage {
    Event(EventMessage),
    ExchangeStatus(ExchangeStatusMessage),
    MarketHistory(HistoryMessage),
    Markets(MarketsMessage),
    Market(MarketMessage),
    MarketOrderbook(OrderbookMessage),
    Series(SeriesMessage),
    Trades(TradesMessage),
    Login(LoginMessage),
    Logout(LogoutMessage),
    Error(ErrorMessage),
}
