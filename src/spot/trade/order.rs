use rust_decimal::Decimal;

use crate::client::Client;
use crate::errors::*;
use crate::rest_model::*;
use crate::util::*;

/// 测试下单
/// https://binance-docs.github.io/apidocs/spot/cn/#trade-2
pub struct TestPlaceOrderBuilder<'a> {
    client: &'a Client,
    playload: OrderRequest,
    recv_window: u64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    /// A unique id for the order, automatically generated if not sent.
    pub new_client_order_id: Option<String>,
    /// Used with stop loss, stop loss limit, take profit and take profit limit order types.
    pub stop_price: Option<f64>,
    /// Used with limit, stop loss limit and take profit limit to create an iceberg order.
    pub iceberg_qty: Option<f64>,
    /// Set the response json, market and limit default to full others to ack.
    pub new_order_resp_type: Option<OrderResponse>,
}

impl<'a> TestPlaceOrderBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String, side: OrderSide, order_type: OrderType) -> Self {
        Self {
            client,
            playload: OrderRequest {
                symbol,
                side,
                order_type,
                ..Default::default()
            },
            recv_window: 0,
        }
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.playload.time_in_force = Some(time_in_force);
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.playload.quantity = Some(quantity);
        self
    }

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.playload.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.playload.price = Some(price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: String) -> Self {
        self.playload.new_client_order_id = Some(new_client_order_id);
        self
    }

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.playload.stop_price = Some(stop_price);
        self
    }

    pub fn iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.playload.iceberg_qty = Some(iceberg_qty);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponse) -> Self {
        self.playload.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub async fn send(self) -> Result<Empty> {
        self.client
            .post_signed_p("/api/v3/order/test", &self.playload, self.recv_window)
            .await
    }
}

/// 下单
/// https://binance-docs.github.io/apidocs/spot/cn/#trade-3
pub struct PlaceOrderBuilder<'a> {
    client: &'a Client,
    playload: OrderRequest,
    recv_window: u64,
}

impl<'a> PlaceOrderBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String, side: OrderSide, order_type: OrderType) -> Self {
        Self {
            client,
            playload: OrderRequest {
                symbol,
                side,
                order_type,
                ..Default::default()
            },
            recv_window: 0,
        }
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.playload.time_in_force = Some(time_in_force);
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.playload.quantity = Some(quantity);
        self
    }

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.playload.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.playload.price = Some(price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: String) -> Self {
        self.playload.new_client_order_id = Some(new_client_order_id);
        self
    }

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.playload.stop_price = Some(stop_price);
        self
    }

    pub fn iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.playload.iceberg_qty = Some(iceberg_qty);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponse) -> Self {
        self.playload.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub async fn send(self) -> Result<PlaceOrder> {
        self.client
            .post_signed_p("/api/v3/order", &self.playload, self.recv_window)
            .await
    }
}

/// 撤销订单
/// https://binance-docs.github.io/apidocs/spot/cn/#trade-4
pub struct CancelOrderBuilder<'a> {
    client: &'a Client,
    playload: CancelOrderRequest,
    recv_window: u64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub order_id: Option<String>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
}

impl<'a> CancelOrderBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            playload: CancelOrderRequest {
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

    pub fn new_client_order_id(mut self, new_client_order_id: String) -> Self {
        self.playload.new_client_order_id = Some(new_client_order_id);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub async fn send(self) -> Result<OrderCanceled> {
        self.client
            .delete_signed_p("/api/v3/order", &self.playload, self.recv_window)
            .await
    }
}

/// 撤销单一交易对的所有挂单
/// https://binance-docs.github.io/apidocs/spot/cn/#trade-5
pub struct CancelAllOpenOrdersBuilder<'a> {
    client: &'a Client,
    symbol: String,
    recv_window: u64,
}

impl<'a> CancelAllOpenOrdersBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String) -> Self {
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

    pub async fn send(self) -> Result<Vec<OrderCanceled>> {
        let data = self.get_params()?;
        self.client.delete_signed("/api/v3/openOrders", data.as_str()).await
    }
}

/// 撤消挂单再下单
/// https://binance-docs.github.io/apidocs/spot/cn/#trade-6
pub struct CancelReplaceBuilder<'a> {
    client: &'a Client,
    playload: CancelReplaceRequest,
    recv_window: u64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceRequest {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub cancel_replace_mode: CancelReplaceMode,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    pub cancel_new_client_order_id: Option<String>,
    pub cancel_orig_client_order_id: Option<String>,
    pub cancel_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub iceberg_qty: Option<f64>,
    pub new_order_resp_type: Option<OrderResponse>,
}

impl<'a> CancelReplaceBuilder<'a> {
    pub fn new(
        client: &'a Client,
        symbol: String,
        side: OrderSide,
        order_type: OrderType,
        cancel_replace_mode: CancelReplaceMode,
    ) -> Self {
        Self {
            client,
            playload: CancelReplaceRequest {
                symbol,
                side,
                order_type,
                cancel_replace_mode,
                ..Default::default()
            },
            recv_window: 0,
        }
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.playload.time_in_force = Some(time_in_force);
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.playload.quantity = Some(quantity);
        self
    }

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.playload.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.playload.price = Some(price);
        self
    }

    pub fn cancel_new_client_order_id(mut self, cancel_new_client_order_id: String) -> Self {
        self.playload.cancel_new_client_order_id = Some(cancel_new_client_order_id);
        self
    }

    pub fn cancel_orig_client_order_id(mut self, cancel_orig_client_order_id: String) -> Self {
        self.playload.cancel_orig_client_order_id = Some(cancel_orig_client_order_id);
        self
    }

    pub fn cancel_order_id(mut self, cancel_order_id: String) -> Self {
        self.playload.cancel_order_id = Some(cancel_order_id);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: String) -> Self {
        self.playload.new_client_order_id = Some(new_client_order_id);
        self
    }

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.playload.stop_price = Some(stop_price);
        self
    }

    pub fn iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.playload.iceberg_qty = Some(iceberg_qty);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponse) -> Self {
        self.playload.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub async fn send(self) -> Result<OrderCanceledReplaced> {
        self.client
            .post_signed_p("/api/v3/order/cancelReplace", &self.playload, self.recv_window)
            .await
    }
}

/***
 * response body
 ***/

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub commission: Decimal,
    pub commission_asset: String,
    pub trade_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder {
    pub symbol: String,
    #[serde(with = "u64_or_string")]
    pub order_id: String,
    pub client_order_id: String,
    pub transact_time: u64,
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
    pub working_time: u64,
    pub fills: Vec<Fill>,
    pub self_trade_prevention_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: String,
    #[serde(with = "u64_or_string")]
    pub order_id: String,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub side: String,
    pub self_trade_prevention_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceledReplaced {
    pub cancel_result: String,
    pub new_order_result: String,
    pub cancel_response: OrderCanceled,
    pub new_order_response: PlaceOrder,
}
