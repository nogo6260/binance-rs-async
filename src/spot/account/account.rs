use crate::client::Client;

use super::query::{
    QueryAccountBuilder, QueryAllOrdersBuilder, QueryMyTradesBuilder, QueryOpenOrdersBuilder, QueryOrderBuilder,
};

pub struct Account {
    pub client: Client,
    pub recv_window: u64,
}

impl Account {
    /// # 查询订单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.account.get_order("BTCUSDT")).order_id("order id");
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-31
    pub fn get_order<S>(&self, symbol: S) -> QueryOrderBuilder
    where
        S: AsRef<str>,
    {
        QueryOrderBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 当前挂单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.account.get_open_orders("BTCUSDT");
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-32
    pub fn get_open_orders<S>(&self, symbol: S) -> QueryOpenOrdersBuilder
    where
        S: AsRef<str>,
    {
        QueryOpenOrdersBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 查询所有订单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.account.get_all_orders("BTCUSDT");
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-33
    pub fn get_all_orders<S>(&self, symbol: S) -> QueryAllOrdersBuilder
    where
        S: AsRef<str>,
    {
        QueryAllOrdersBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 账户信息
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.account.get_account();
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-34
    pub fn get_account(&self) -> QueryAccountBuilder {
        QueryAccountBuilder::new(&self.client)
    }

    /// # 成交历史
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let bulider = spot.account.get_my_trades("BTCUSDT");
    /// let data = tokio_test::block_on(bulider.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-35
    pub fn get_my_trades<S>(&self, symbol: S) -> QueryMyTradesBuilder
    where
        S: AsRef<str>,
    {
        QueryMyTradesBuilder::new(&self.client, symbol.as_ref().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Binance;
    use crate::config::Config;
    use crate::Spot;
    use dotenv::dotenv;
    use std::env;

    fn env() -> (String, String) {
        dotenv().ok();
        (env::var("API_KEY").unwrap(), env::var("API_SECRET").unwrap())
    }

    #[test]
    fn test_get_account() {
        let (key, secret) = env();
        let spot: Spot = Binance::new_with_config(Some(key), Some(secret), &Config::testnet());
        let builder = spot.account.get_account().send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());
    }
}
