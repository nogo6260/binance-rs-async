use rust_decimal::Decimal;

use crate::errors::*;
use crate::util::string_to_decimal;
use crate::{client::Client, util::build_request};

/// 近期成交列表
/// https://binance-docs.github.io/apidocs/spot/cn/#2c5e424c25
pub struct TradesBuilder<'a> {
    client: &'a Client,
    symbol: String,
    limit: Option<u16>,
}

impl<'a> TradesBuilder<'a> {
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            symbol,
            limit: None,
        }
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    fn get_params(&self) -> String {
        let parameters = IntoIterator::into_iter([
            Some(("symbol", self.symbol.to_string())),
            self.limit.map(|l| ("limit", l.to_string())),
        ])
        .flatten();
        build_request(parameters)
    }

    pub async fn send(self) -> Result<Vec<Trade>> {
        let data = self.get_params();
        self.client.get("/api/v3/trades", Some(data.as_str())).await
    }
}

/// 查询历史成交
/// https://binance-docs.github.io/apidocs/spot/cn/#5221bade13
pub struct HistoricalTradesBuilder<'a> {
    client: &'a Client,
    symbol: String,
    limit: Option<u16>,
    from_id: Option<u64>,
}

impl<'a> HistoricalTradesBuilder<'a> {
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            symbol,
            limit: None,
            from_id: None,
        }
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    fn get_params(&self) -> String {
        let parameters = IntoIterator::into_iter([
            Some(("symbol", self.symbol.to_string())),
            self.limit.map(|l| ("limit", l.to_string())),
            self.from_id.map(|l| ("fromId", l.to_string())),
        ])
        .flatten();
        build_request(parameters)
    }

    pub async fn send(self) -> Result<Vec<Trade>> {
        let data = self.get_params();
        self.client.get("/api/v3/historicalTrades", Some(data.as_str())).await
    }
}

/// 近期成交(归集)
/// https://binance-docs.github.io/apidocs/spot/cn/#c59e471e81
pub struct AggTradesBuilder<'a> {
    client: &'a Client,
    symbol: String,
    from_id: Option<u64>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u16>,
}

impl<'a> AggTradesBuilder<'a> {
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            symbol,
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }
    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
       self.start_time = Some(start_time);
       self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
       self.end_time = Some(end_time);
       self
    }

    pub fn limit(mut self, limit: u16) -> Self {
       self.limit = Some(limit);
       self
    }
    
    fn get_params(&self) -> String {
        let parameters = IntoIterator::into_iter([
            Some(("symbol", self.symbol.to_string())),
            self.from_id.map(|l| ("fromId", l.to_string())),
            self.start_time.map(|l| ("startTime", l.to_string())),
            self.end_time.map(|l| ("endTime", l.to_string())),
            self.limit.map(|l| ("limit", l.to_string())),
        ])
        .flatten();
        build_request(parameters)
    }

    pub async fn send(self) -> Result<Vec<AggTrade>> {
        let data = self.get_params();
        self.client.get("/api/v3/aggTrades", Some(data.as_str())).await
    }
}

/****
    response body
****/

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub qty: Decimal,
    pub time: u64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggTrade {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "a")]
    pub agg_id: u64,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "p", with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(rename = "q", with = "string_to_decimal")]
    pub qty: Decimal,
}
