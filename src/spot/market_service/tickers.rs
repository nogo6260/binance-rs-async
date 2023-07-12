use rust_decimal::Decimal;

use crate::client::Client;
use crate::errors::*;
use crate::util::*;

/// 24hr 价格变动情况
/// https://binance-docs.github.io/apidocs/spot/cn/#24hr
pub struct Ticker24hBuilder<'a> {
    client: &'a Client,
    symbol: String,
}

impl<'a> Ticker24hBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String) -> Self {
        Self { client, symbol }
    }

    fn get_params(&self) -> String {
        let parameters = [("symbol", self.symbol.to_string())];
        build_request(parameters)
    }

    pub async fn send(self) -> Result<Ticker> {
        let data = self.get_params();
        self.client.get("/api/v3/ticker/24hr", Some(data.as_str())).await
    }
}

/// 24hr 价格变动情况
/// https://binance-docs.github.io/apidocs/spot/cn/#24hr
pub struct Ticker24hMultiBuilder<'a> {
    client: &'a Client,
    symbols: Option<Vec<String>>,
}

impl<'a> Ticker24hMultiBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client, symbols: None }
    }

    pub fn symbols(mut self, symbols: Vec<String>) -> Self {
        self.symbols = Some(symbols);
        self
    }

    fn get_params(&self) -> Option<String> {
        let data = self.symbols.clone();
        if let Some(val) = data {
            let symbols = serde_json::to_string(&val).unwrap();
            let parameters = [("symbols", symbols)];
            Some(build_request(parameters))
        } else {
            None
        }
    }

    pub async fn send(self) -> Result<Vec<Ticker>> {
        let data = self.get_params();
        self.client.get("/api/v3/ticker/24hr", data.as_deref()).await
    }
}

/// 最新价格
/// https://binance-docs.github.io/apidocs/spot/cn/#8ff46b58de
pub struct LastPriceBuilder<'a> {
    client: &'a Client,
    symbol: String,
}

impl<'a> LastPriceBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String) -> Self {
        Self { client, symbol }
    }

    fn get_params(&self) -> String {
        let parameters = [("symbol", self.symbol.to_string())];
        build_request(parameters)
    }

    pub async fn send(self) -> Result<LastPrice> {
        let data = self.get_params();
        self.client.get("/api/v3/ticker/price", Some(data.as_str())).await
    }
}

/// 最新价格
/// https://binance-docs.github.io/apidocs/spot/cn/#8ff46b58de
pub struct LastPriceMultiBuilder<'a> {
    client: &'a Client,
    symbols: Option<Vec<String>>,
}

impl<'a> LastPriceMultiBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client, symbols: None }
    }

    pub fn symbols(mut self, symbols: Vec<String>) -> Self {
        self.symbols = Some(symbols);
        self
    }

    fn get_params(&self) -> Option<String> {
        let data = self.symbols.clone();
        if let Some(val) = data {
            let symbols = serde_json::to_string(&val).unwrap();
            let parameters = [("symbols", symbols)];
            Some(build_request(parameters))
        } else {
            None
        }
    }

    pub async fn send(self) -> Result<Vec<LastPrice>> {
        let data = self.get_params();
        self.client.get("/api/v3/ticker/price", data.as_deref()).await
    }
}

/// 最新价格
/// https://binance-docs.github.io/apidocs/spot/cn/#5393cd07b4
pub struct BookTickerBuilder<'a> {
    client: &'a Client,
    symbol: String,
}

impl<'a> BookTickerBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String) -> Self {
        Self { client, symbol }
    }

    fn get_params(&self) -> String {
        let parameters = [("symbol", self.symbol.to_string())];
        build_request(parameters)
    }

    pub async fn send(self) -> Result<BookTicker> {
        let data = self.get_params();
        self.client.get("/api/v3/ticker/bookTicker", Some(data.as_str())).await
    }
}

/// 最新价格
/// https://binance-docs.github.io/apidocs/spot/cn/#5393cd07b4
pub struct BookTickerMultiBuilder<'a> {
    client: &'a Client,
    symbols: Option<Vec<String>>,
}

impl<'a> BookTickerMultiBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client, symbols: None }
    }

    pub fn symbols(mut self, symbols: Vec<String>) -> Self {
        self.symbols = Some(symbols);
        self
    }

    fn get_params(&self) -> Option<String> {
        let data = self.symbols.clone();
        if let Some(val) = data {
            let symbols = serde_json::to_string(&val).unwrap();
            let parameters = [("symbols", symbols)];
            Some(build_request(parameters))
        } else {
            None
        }
    }

    pub async fn send(self) -> Result<Vec<BookTicker>> {
        let data = self.get_params();
        self.client.get("/api/v3/ticker/bookTicker", data.as_deref()).await
    }
}

/***
 * response body
***/

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_to_decimal")]
    pub prev_close_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub last_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub bid_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub open_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub high_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub low_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub volume: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LastPrice {
    pub symbol: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    pub symbol: String,
    #[serde(with = "string_to_decimal")]
    pub bid_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub bid_qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask_qty: Decimal,
}
