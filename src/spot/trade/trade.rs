use crate::client::Client;
use crate::rest_model::*;

use super::order::{
    CancelAllOpenOrdersBuilder, CancelOrderBuilder, CancelReplaceBuilder, PlaceOrderBuilder, TestPlaceOrderBuilder,
};

pub struct Trade {
    pub client: Client,
    pub recv_window: u64,
}

impl Trade {
    /// # 测试下单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.trade.test_place_order("BTCUSDT",OrderSide::Buy,OrderType::Limit)
    ///    .time_in_force(TimeInForce::GTC)
    ///    .price(25000.0)
    ///    .quantity(0.001)
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-2
    pub fn test_place_order<S1, S2, S3>(&self, symbol: S1, side: S2, order_type: S3) -> TestPlaceOrderBuilder
    where
        S1: AsRef<str>,
        S2: Into<OrderSide>,
        S3: Into<OrderType>,
    {
        TestPlaceOrderBuilder::new(
            &self.client,
            symbol.as_ref().to_string(),
            side.into(),
            order_type.into(),
        )
    }

    /// # 下单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.trade.test_place_order("BTCUSDT",OrderSide::Buy,OrderType::Limit)
    ///    .time_in_force(TimeInForce::GTC)
    ///    .price(25000.0)
    ///    .quantity(0.001)
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-3
    pub fn place_order<S1, S2, S3>(&self, symbol: S1, side: S2, order_type: S3) -> PlaceOrderBuilder
    where
        S1: AsRef<str>,
        S2: Into<OrderSide>,
        S3: Into<OrderType>,
    {
        PlaceOrderBuilder::new(
            &self.client,
            symbol.as_ref().to_string(),
            side.into(),
            order_type.into(),
        )
    }

    /// # 撤销订单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.trade.test_place_order("BTCUSDT",OrderSide::Buy,OrderType::Limit)
    ///    .time_in_force(TimeInForce::GTC)
    ///    .price(25000.0)
    ///    .quantity(0.001)
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-4
    pub fn cancel_order<S>(&self, symbol: S) -> CancelOrderBuilder
    where
        S: AsRef<str>,
    {
        CancelOrderBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 撤销单一交易对的所有挂单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.trade.cancel_all_open_orders("BTCUSDT")
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-5
    pub fn cancel_all_open_orders<S>(&self, symbol: S) -> CancelAllOpenOrdersBuilder
    where
        S: AsRef<str>,
    {
        CancelAllOpenOrdersBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 撤消挂单再下单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.trade.cancel_replace_order("BTCUSDT")
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-6
    pub fn cancel_replace_order<S1, S2, S3, S4>(
        &self,
        symbol: S1,
        side: S2,
        order_type: S3,
        cancel_replace_mode: S4,
    ) -> CancelReplaceBuilder
    where
        S1: AsRef<str>,
        S2: Into<OrderSide>,
        S3: Into<OrderType>,
        S4: Into<CancelReplaceMode>,
    {
        CancelReplaceBuilder::new(
            &self.client,
            symbol.as_ref().to_string(),
            side.into(),
            order_type.into(),
            cancel_replace_mode.into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Binance;
    use crate::config::Config;
    use crate::rest_model::*;
    use crate::Spot;
    use dotenv::dotenv;
    use std::env;

    fn env() -> (String,String) {
        dotenv().ok();
       (env::var("API_KEY").unwrap(), env::var("API_SECRET").unwrap())
    }

    #[test]
    fn test_test_place_order() {
        let (key,secret) = env();
        let spot: Spot = Binance::new_with_config(Some(key), Some(secret), &Config::testnet());
        let builder = spot
            .trade
            .test_place_order("BTCUSDT", OrderSide::Buy, OrderType::Limit)
            .time_in_force(TimeInForce::GTC)
            .price(25000.0)
            .quantity(0.001)
            .send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());
    }

    #[test]
    fn test_place_order() {
        let (key,secret) = env();
        let spot: Spot = Binance::new_with_config(Some(key), Some(secret), &Config::testnet());
        let builder = spot
            .trade
            .place_order("BTCUSDT", OrderSide::Buy, OrderType::Limit)
            .time_in_force(TimeInForce::GTC)
            .price(25000.0)
            .quantity(0.001)
            .send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());
    }

    #[test]
    fn test_cancel_order() {
        let (key,secret) = env();
        let spot: Spot = Binance::new_with_config(Some(key), Some(secret), &Config::testnet());
        let builder = spot.trade.cancel_order("BTCUSDT").order_id("3229491".into()).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());
    }

    #[test]
    fn test_cancel_all_open_orders() {
        let (key,secret) = env();
        let spot: Spot = Binance::new_with_config(Some(key), Some(secret), &Config::testnet());
        let builder = spot.trade.cancel_all_open_orders("BTCUSDT").send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());
    }

    #[test]
    fn test_cancel_replace_order() {
        let (key,secret) = env();
        let spot: Spot = Binance::new_with_config(Some(key), Some(secret), &Config::testnet());
        let builder = spot
            .trade
            .cancel_replace_order(
                "BTCUSDT",
                OrderSide::Buy,
                OrderType::Limit,
                CancelReplaceMode::StopOnFailure,
            )
            .cancel_order_id("3251081".into())
            .time_in_force(TimeInForce::GTC)
            .new_client_order_id("test_cancel_replace_order".into())
            .price(24000.0)
            .quantity(0.001)
            .send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());
    }
}
