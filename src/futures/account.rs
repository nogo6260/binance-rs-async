use std::collections::BTreeMap;
use std::fmt;

use serde::Serializer;

use crate::account::OrderCancellation;
use crate::client::Client;
use crate::errors::*;
use crate::futures::futures_type::FuturesType;
use crate::rest_model::{OrderSide, TimeInForce};
use crate::rest_model::{PairAndWindowQuery, PairQuery};
use crate::util::*;

use super::router::FuturesRoute;
use super::rest_model::{
    AccountBalance, AccountInformation, CanceledOrder, ChangeLeverageResponse, Order, OrderType, Position,
    PositionSide, Transaction, WorkingType,
};

pub struct FuturesAccount<T> {
    pub client: Client,
    pub recv_window: u64,
    pub router: fn(FuturesRoute) -> String,
    pub _marker: std::marker::PhantomData<T>,
}

/// Serialize bool as str
fn serialize_as_str<S, T>(t: &T, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: fmt::Display,
{
    serializer.collect_str(t)
}

/// Serialize opt bool as str
fn serialize_opt_as_uppercase<S, T>(t: &Option<T>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
{
    match *t {
        Some(ref v) => serializer.serialize_some(&v.to_string().to_uppercase()),
        None => serializer.serialize_none(),
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "quantity")]
    pub quantity: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub close_position: Option<bool>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    #[serde(serialize_with = "serialize_opt_as_uppercase")]
    pub price_protect: Option<bool>,
    pub new_client_order_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ChangePositionModeRequest {
    #[serde(serialize_with = "serialize_as_str")]
    pub dual_side_position: bool,
}

impl<T> FuturesAccount<T>
    where T: FuturesType,
{

    fn get_api(&self, f: FuturesRoute)-> String{
        (self.router)(f)
    }

    pub async fn place_order(&self, order: OrderRequest) -> Result<Transaction> {
        self.client
            .post_signed_p(self.get_api(FuturesRoute::Order).as_str(), order, self.recv_window)
            .await
    }

    pub async fn get_open_orders(&self, symbol: impl Into<String>) -> Result<Vec<Order>> {
        let payload = build_signed_request_p([("symbol", symbol.into())], self.recv_window)?;
        self.client.get_signed(self.get_api(FuturesRoute::OpenOrders).as_str(), &payload).await
    }

    pub async fn limit_buy(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<f64>,
        price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            quantity: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_client_order_id: None,
        };
        self.place_order(order).await
    }

    pub async fn limit_sell(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<f64>,
        price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            quantity: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_client_order_id: None,
        };
        self.place_order(order).await
    }

    // Place a MARKET order - BUY
    pub async fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
        where
            S: Into<String>,
            F: Into<f64>,
    {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_client_order_id: None,
        };
        self.place_order(order).await
    }

    // Place a MARKET order - SELL
    pub async fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
        where
            S: Into<String>,
            F: Into<f64>,
    {
        let order: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_client_order_id: None,
        };
        self.place_order(order).await
    }

    /// Place a cancellation order
    pub async fn cancel_order(&self, o: OrderCancellation) -> Result<CanceledOrder> {
        let recv_window = o.recv_window.unwrap_or(self.recv_window);
        self.client.delete_signed_p(self.get_api(FuturesRoute::Order).as_str(), &o, recv_window).await
    }

    pub async fn position_information<S>(&self, symbol: S) -> Result<Vec<Position>>
        where
            S: Into<String>,
    {
        self.client
            .get_signed_p(
                self.get_api(FuturesRoute::PositionRisk).as_str(),
                Some(PairAndWindowQuery {
                    symbol: symbol.into(),
                    recv_window: self.recv_window,
                }),
                self.recv_window,
            )
            .await
    }

    pub async fn account_information(&self) -> Result<AccountInformation> {
        // needs to be changed to smth better later
        let payload = build_signed_request(BTreeMap::<String, String>::new(), self.recv_window)?;
        self.client.get_signed_d(self.get_api(FuturesRoute::Account).as_str(), &payload).await
    }

    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::<String, String>::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed_d(self.get_api(FuturesRoute::Balance).as_str(), request.as_str()).await
    }

    pub async fn change_initial_leverage<S>(&self, symbol: S, leverage: u8) -> Result<ChangeLeverageResponse>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.post_signed_d(self.get_api(FuturesRoute::ChangeInitialLeverage).as_str(), request.as_str()).await
    }

    pub async fn change_position_mode(&self, dual_side_position: bool) -> Result<()> {
        self.client
            .post_signed_p(
                self.get_api(FuturesRoute::PositionSide).as_str(),
                ChangePositionModeRequest { dual_side_position },
                self.recv_window,
            )
            .await?;
        Ok(())
    }

    pub async fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<()>
        where
            S: Into<String>,
    {
        self.client
            .delete_signed_p(
                self.get_api(FuturesRoute::AllOpenOrders).as_str(),
                PairQuery { symbol: symbol.into() },
                self.recv_window,
            )
            .await?;
        Ok(())
    }
}
