use crate::client::Client;

use super::depth::DepthBuilder;
use super::klines::KlinesBuilder;
use super::tickers::{BookTickerBuilder, BookTickerMultiBuilder, LastPriceBuilder, LastPriceMultiBuilder, Ticker24hBuilder, Ticker24hMultiBuilder};
use super::trades::{AggTradesBuilder, HistoricalTradesBuilder, TradesBuilder};

pub struct Market {
    pub(crate) client: Client,
    pub(crate) recv_window: u64,
}

impl Market {
    /// # 深度信息
    /// `limit` 可选值： 5, 10, 20, 50, 100, 500, 1000, 5000
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_depth("BTCUSDT").limit(50);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#38a975b802
    pub fn get_depth<S>(&self, symbol: S) -> DepthBuilder
        where
            S: AsRef<str>,
    {
        DepthBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 近期成交列表
    /// `limit` 可选值： 0-1000
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_trades("BTCUSDT").limit(50);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#2c5e424c25
    pub fn get_trades<S>(&self, symbol: S) -> TradesBuilder
        where
            S: AsRef<str>,
    {
        TradesBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 查询历史成交
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_historical_trades("BTCUSDT").limit(50);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#5221bade13
    pub fn get_historical_trades<S>(&self, symbol: S) -> HistoricalTradesBuilder
        where
            S: AsRef<str>,
    {
        HistoricalTradesBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 近期成交(归集)
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_agg_trades("BTCUSDT").limit(50);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#c59e471e81
    pub fn get_agg_trades<S>(&self, symbol: S) -> AggTradesBuilder
        where
            S: AsRef<str>,
    {
        AggTradesBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # K线数据
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_klines("BTCUSDT","1m").limit(50);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#k
    pub fn get_klines<S1, S2>(&self, symbol: S1, interval: S2) -> KlinesBuilder
        where
            S1: AsRef<str>,
            S2: AsRef<str>,
    {
        KlinesBuilder::new(&self.client, symbol.as_ref().to_string(), interval.as_ref().to_string())
    }

    /// # 24hr 价格变动情况
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_ticker_24h("BTCUSDT");
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#24hr
    pub fn get_ticker_24h<S>(&self, symbol: S) -> Ticker24hBuilder
        where
            S: AsRef<str>,
    {
        Ticker24hBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 24hr 价格变动情况（多交易对）
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_ticker_24h_multi().symbols(vec!["BTCUSDT","ETHUSDT"]);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#24hr
    pub fn get_ticker_24h_multi(&self) -> Ticker24hMultiBuilder {
        Ticker24hMultiBuilder::new(&self.client)
    }

    /// # 最新价格
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_last_price("BTCUSDT");
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#8ff46b58de
    pub fn get_last_price<S>(&self, symbol: S) -> LastPriceBuilder
        where
            S: AsRef<str>,
    {
        LastPriceBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 最新价格（多交易对）
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_last_price_multi().symbols(vec!["BTCUSDT","ETHUSDT"]);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#8ff46b58de
    pub fn get_last_price_multi(&self) -> LastPriceMultiBuilder {
        LastPriceMultiBuilder::new(&self.client)
    }


    /// # 最有挂单
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_book_ticker("BTCUSDT");
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#8ff46b58de
    pub fn get_book_ticker<S>(&self, symbol: S) -> BookTickerBuilder
        where
            S: AsRef<str>,
    {
        BookTickerBuilder::new(&self.client, symbol.as_ref().to_string())
    }

    /// # 最有挂单（多交易对）
    /// ## Example
    /// ``` rust
    /// use binance::api::Binance;
    /// use binance::Spot;
    /// let spot:Spot = Binance::new(None,None);
    /// let builder = spot.market.get_last_price_multi().symbols(vec!["BTCUSDT","ETHUSDT"]);
    /// let data = tokio_test::block_on(builder.send());
    /// assert!(data.is_ok(), "{:?}", data);
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#8ff46b58de
    pub fn get_book_ticker_multi(&self) -> BookTickerMultiBuilder {
        BookTickerMultiBuilder::new(&self.client)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Binance;
    use crate::config::Config;
    use crate::Spot;

    #[test]
    fn market_test() {
        let spot: Spot = Binance::new_with_config(None, None, &Config::testnet());
        let builder = spot.market.get_depth("ETHUSDT").limit(5).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_trades("ETHUSDT").limit(5).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_historical_trades("ETHUSDT").limit(5).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_agg_trades("ETHUSDT").limit(5).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_klines("ETHUSDT", "1m").limit(5).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_ticker_24h("ETHUSDT").send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_ticker_24h_multi().symbols(vec!["ETHUSDT", "BTCUSDT"]).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_last_price("ETHUSDT").send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_last_price_multi().symbols(vec!["ETHUSDT", "BTCUSDT"]).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_book_ticker("ETHUSDT").send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());

        let builder = spot.market.get_book_ticker_multi().symbols(vec!["ETHUSDT", "BTCUSDT"]).send();
        let resp = tokio_test::block_on(builder);
        println!("{:?}", resp.unwrap());
        //assert!(depth.is_ok(), "{:?}", depth);
    }
}
