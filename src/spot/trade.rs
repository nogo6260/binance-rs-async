use crate::client::Client;

pub struct Trade {
    pub client: Client,
    pub recv_window: u64,
}

impl Trade {
    /// 测试下单
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-2
    pub async fn test_place_order(){
        todo!()
    }

    /// 下单
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-3
    pub async fn place_order(){
        todo!()
    }

    /// 撤单
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-4
    pub async fn cancel_order(){
        todo!()
    }

    /// 撤单
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-5
    pub async fn cancel_all_open_orders(){
        todo!()
    }

    /// 撤单后再下单
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#trade-6
    pub async fn cancel_replace_orders(){
        todo!()
    }
}