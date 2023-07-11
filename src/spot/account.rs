use crate::client::Client;
use crate::errors::*;
use crate::spot::request::*;
use crate::spot::response::Order;
use crate::util::*;

pub struct Account {
    pub client: Client,
    pub recv_window: u64,
}

impl Account {
    /// 查询订单
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-31
    pub async fn get_order(&self, p: GetOrderRequest) -> Result<Order> {
        let recv_window = p.recv_window.unwrap_or(self.recv_window);
        let request = build_signed_request_p(p, recv_window)?;
        self.client.get_signed("/api/v3/order", &request).await
    }

    /// 当前订单
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-32
    pub async fn get_all_open_orders(&self, query: OrdersQuery) -> Result<Vec<Order>> {
        let recv_window = query.recv_window.unwrap_or(self.recv_window);
        let request = build_signed_request_p(query, recv_window)?;
        self.client.get_signed("/api/v3/allOrders", &request).await
    }

    /// 账户信息
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-34
    pub async fn get_account() {
        todo!()
    }

    /// 成交历史
    /// # Example
    /// ``` rust
    /// use binance;
    /// ```
    /// https://binance-docs.github.io/apidocs/spot/cn/#user_data-35
    pub async fn get_trade_history() {
        todo!()
    }
}

