pub mod orderbook;
pub mod series;
pub mod event;
pub mod market;
pub mod trade;
pub mod history;
pub mod exchange;
pub mod lib;
pub mod error;
pub mod login;
pub mod logout;

use serde::{Serialize, Deserialize};


use self::{event::{EventMessage}, exchange::{ExchangeStatusMessage}, history::{HistoryMessage}, market::{MarketsMessage, MarketMessage}, orderbook::{OrderbookMessage}, series::{SeriesMessage}, trade::{TradesMessage}, error::ErrorMessage, login::LoginMessage, logout::LogoutMessage};

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