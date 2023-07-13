use rust_decimal::Decimal;

use crate::client::Client;
use crate::errors::*;
use crate::rest_model::{AccountType, OrderSide, OrderStatus, OrderType, TimeInForce};
use crate::util::*;

/// 查询订单
/// https://binance-docs.github.io/apidocs/spot/cn/#user_data-31
pub struct QueryOrderBuilder<'a> {
    client: &'a Client,
    playload: OrderRequest,
    recv_window: u64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct OrderRequest {
    symbol: String,
    order_id: Option<String>,
    orig_client_order_id: Option<String>,
}

impl<'a> QueryOrderBuilder<'a> {
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            playload: OrderRequest {
                symbol,
                ..Default::default()
            },
            recv_window: 0,
        }
    }

    pub fn order_id(mut self, order_id: String) -> Self {
        self.playload.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: String) -> Self {
        self.playload.orig_client_order_id = Some(orig_client_order_id);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub async fn send(self) -> Result<QueryOrder> {
        let data = build_signed_request_p(&self.playload, self.recv_window)?;
        self.client.get_signed("/api/v3/order", data.as_str()).await
    }
}

/// 当前挂单
/// https://binance-docs.github.io/apidocs/spot/cn/#user_data-32
pub struct QueryOpenOrdersBuilder<'a> {
    client: &'a Client,
    symbol: String,
    recv_window: u64,
}

impl<'a> QueryOpenOrdersBuilder<'a> {
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            symbol,
            recv_window: 0,
        }
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    fn get_params(&self) -> Result<String> {
        let parameters = [("symbol", self.symbol.to_string())];
        build_signed_request(parameters, self.recv_window)
    }

    pub async fn send(self) -> Result<Vec<QueryOrder>> {
        let data = self.get_params()?;
        self.client.get_signed("/api/v3/openOrders", data.as_str()).await
    }
}

/// 查询所有订单
/// https://binance-docs.github.io/apidocs/spot/cn/#user_data-33
pub struct QueryAllOrdersBuilder<'a> {
    client: &'a Client,
    playload: AllOrdersRequest,
    recv_window: u64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AllOrdersRequest {
    symbol: String,
    order_id: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u16>,
}

impl<'a> QueryAllOrdersBuilder<'a> {
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            playload: AllOrdersRequest {
                symbol,
                ..Default::default()
            },
            recv_window: 0,
        }
    }

    pub fn order_id(mut self, order_id: String) -> Self {
        self.playload.order_id = Some(order_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.playload.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.playload.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.playload.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub async fn send(self) -> Result<Vec<QueryOrder>> {
        let data = build_signed_request_p(&self.playload, self.recv_window)?;
        self.client.get_signed("/api/v3/allOrders", data.as_str()).await
    }
}

/// 账户信息
/// https://binance-docs.github.io/apidocs/spot/cn/#user_data-34
pub struct QueryAccountBuilder<'a> {
    client: &'a Client,
    recv_window: u64,
}

impl<'a> QueryAccountBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client, recv_window: 0 }
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    fn get_params(&self) -> Result<String> {
        let parameters = [("", "")];
        build_signed_request(parameters, self.recv_window)
    }

    pub async fn send(self) -> Result<AccountInformation> {
        let data = self.get_params()?;
        self.client.get_signed("/api/v3/account", data.as_str()).await
    }
}

/// 账户成交历史
/// https://binance-docs.github.io/apidocs/spot/cn/#user_data-35
pub struct QueryMyTradesBuilder<'a> {
    client: &'a Client,
    playload: MyTradeRequest,
    recv_window: u64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MyTradeRequest {
    symbol: String,
    order_id: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    from_id: Option<u64>,
    limit: Option<u16>,
}

impl<'a> QueryMyTradesBuilder<'a> {
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            recv_window: 0,
            playload: MyTradeRequest {
                symbol,
                ..Default::default()
            },
        }
    }

    pub fn order_id(mut self, order_id: String) -> Self {
        self.playload.order_id = Some(order_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.playload.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.playload.end_time = Some(end_time);
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.playload.from_id = Some(from_id);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.playload.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub async fn send(self) -> Result<Vec<TradeHistory>> {
        let data = build_signed_request_p(&self.playload, self.recv_window)?;
        self.client.get_signed("/api/v3/myTrades", data.as_str()).await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrder {
    pub symbol: String,
    #[serde(with = "u64_or_string")]
    pub order_id: String,
    pub order_list_id: i32,
    pub client_order_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub orig_qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub executed_qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    #[serde(with = "string_to_decimal")]
    pub stop_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub iceberg_qty: Decimal,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub working_time: u64,
    #[serde(with = "string_to_decimal")]
    pub orig_quote_order_qty: Decimal,
    pub self_trade_prevention_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub id: u64,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub qty: Decimal,
    pub commission: String,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: f32,
    pub taker_commission: f32,
    pub buyer_commission: f32,
    pub seller_commission: f32,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub account_type: AccountType,
    pub balances: Vec<Balance>,
    pub permissions: Vec<AccountType>,
    pub update_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    #[serde(with = "string_to_decimal")]
    pub free: Decimal,
    #[serde(with = "string_to_decimal")]
    pub locked: Decimal,
}
