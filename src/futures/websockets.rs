use std::env::var;
use std::mem::swap;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

use futures::StreamExt;
use serde_json::{from_str, from_value, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use url::Url;

use crate::config::Config;
use crate::errors::*;
use crate::ws_model::*;

pub fn agg_trade_stream(symbol: &str) -> String { format!("{symbol}@aggTrade") }

pub fn kline_stream(symbol: &str, interval: &str) -> String { format!("{symbol}@kline_{interval}") }

pub fn mini_ticker_stream(symbol: &str) -> String { format!("{symbol}@miniTicker") }

pub fn all_mini_ticker_stream() -> &'static str { "!miniTicker@arr" }

pub fn ticker_stream(symbol: &str) -> String { format!("{symbol}@ticker") }

pub fn all_ticker_stream() -> &'static str { "!ticker@arr" }

pub fn trade_stream(symbol: &str) -> String { format!("{symbol}@trade") }

pub fn book_ticker_stream(symbol: &str) -> String { format!("{symbol}@bookTicker") }

pub fn all_book_ticker_stream() -> &'static str { "!bookTicker" }

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `levels`: 5, 10 or 20
/// * `update_speed`: 1000 or 100
pub fn partial_book_depth_stream(symbol: &str, levels: u16, update_speed: u16) -> String {
    format!("{symbol}@depth{levels}@{update_speed}ms")
}

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `update_speed`: 1000 or 100
pub fn diff_book_depth_stream(symbol: &str, update_speed: u16) -> String { format!("{symbol}@depth@{update_speed}ms") }

fn combined_stream(streams: Vec<String>) -> String { streams.join("/") }


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum FuturesWebsocketEvent {
    #[serde(alias = "aggTrade")]
    AggTrade(Box<TradesEvent>),
    #[serde(alias = "trade")]
    Trade(Box<super::ws_model::TradeEvent>),
    #[serde(alias = "kline")]
    Kline(Box<KlineEvent>),
    #[serde(alias = "24hrTicker")]
    DayTicker(Box<DayTickerEvent>),
    #[serde(alias = "24hrMiniTicker")]
    DayMiniTicker(Box<MiniDayTickerEvent>),
    #[serde(alias = "depthUpdate")]
    DepthOrderBook(Box<DepthOrderBookEvent>),
    #[serde(alias = "outboundAccountPosition")]
    AccountPositionUpdate(Box<AccountPositionUpdate>),
    #[serde(alias = "balanceUpdate")]
    BalanceUpdate(Box<BalanceUpdate>),
    #[serde(alias = "executionReport")]
    OrderUpdate(Box<OrderUpdate>),
    #[serde(alias = "listStatus")]
    ListOrderUpdate(Box<OrderListUpdate>),
}

pub struct FuturesWebSockets<'a> {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(FuturesWebsocketEvent) -> Result<()> + 'a + Send>,
    conf: Config,
}

impl<'a> FuturesWebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> FuturesWebSockets<'a>
        where
            Callback: FnMut(FuturesWebsocketEvent) -> Result<()> + 'a + Send,
    {
        Self::new_with_options(handler, Config::default())
    }

    pub fn new_with_options<Callback>(handler: Callback, conf: Config) -> FuturesWebSockets<'a>
        where
            Callback: FnMut(FuturesWebsocketEvent) -> Result<()> + 'a + Send,
    {
        FuturesWebSockets {
            socket: None,
            handler: Box::new(handler),
            conf,
        }
    }

    /// Connect to multiple websocket endpoints
    /// N.B: WE has to be CombinedStreamEvent
    pub async fn connect_multiple(&mut self, endpoints: Vec<String>) -> Result<()> {
        let mut url = Url::parse(&self.conf.futures_ws_endpoint)?;
        url.path_segments_mut()
            .map_err(|_| Error::UrlParserError(url::ParseError::RelativeUrlWithoutBase))?
            .push("stream");
        url.set_query(Some(&format!("streams={}", combined_stream(endpoints))));

        self.handle_connect(url).await
    }

    /// Connect to a websocket endpoint
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/ws/{}", self.conf.futures_ws_endpoint, endpoint);
        let url = Url::parse(&wss)?;

        self.handle_connect(url).await
    }

    async fn handle_connect(&mut self, url: Url) -> Result<()> {
        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {e}"))),
        }
    }

    /// Disconnect from the endpoint
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub fn socket(&self) -> &Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)> { &self.socket }

    fn message_handler(&mut self, msg: &str) -> Result<()> {
        let value: Value = from_str(msg)?;
        let event = if let Some(data) = value.get("data").cloned() {
            from_value::<FuturesWebsocketEvent>(data)?
        } else {
            from_value::<FuturesWebsocketEvent>(value)?
        };
        (self.handler)(event)
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some((ref mut socket, _)) = self.socket {
                // TODO: return error instead of panic?
                let message = socket.next().await.unwrap()?;

                match message {
                    Message::Text(msg) => {
                        self.message_handler(msg.as_str())?;
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
                    Message::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {e:?}")));
                    }
                }
            }
        }
        Ok(())
    }
}
