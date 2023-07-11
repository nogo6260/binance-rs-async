use crate::client::Client;
use crate::errors::*;
use crate::spot::response::OrderBook;
use crate::util::*;

pub struct Market {
    pub(crate) client: Client,
    pub(crate) recv_window: u64,
}

impl Market {
    fn symbol_request<S>(&self, symbol: S) -> String
        where
            S: AsRef<str>,
    {
        build_request([("symbol", symbol)])
    }

    /// 深度信息
    /// https://binance-docs.github.io/apidocs/spot/cn/#38a975b802
    /// # Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let depth = tokio_test::block_on(spot.market.get_depth("BTCUSDT"));
    /// assert!(depth.is_ok(), "{:?}", depth);
    /// ```
    pub async fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
        where
            S: AsRef<str>,
    {
        let request = self.symbol_request(symbol);
        self.client.get("/api/v3/depth", Some(&request)).await
    }
}


#[cfg(test)]
mod tests {
    use crate::api::Binance;
    use crate::config::Config;
    use crate::Spot;

    #[test]
    fn test_get_depth() {
        let spot: Spot = Binance::new_with_config(None, None, &Config::testnet());
        let depth = tokio_test::block_on(spot.market.get_depth("BTCUSDT"));
        assert!(depth.is_ok(), "{:?}", depth);
    }
}