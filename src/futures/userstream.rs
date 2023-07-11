use crate::client::*;
use crate::errors::*;
use crate::futures::futures_type::FuturesType;
use crate::futures::router::FuturesRoute;
use crate::rest_model::*;

#[derive(Clone)]
pub struct UserStream<T> {
    pub client: Client,
    pub recv_window: u64,
    pub router: fn(FuturesRoute)-> String,
    pub _marker: std::marker::PhantomData<T>,
}

impl<T> UserStream <T>
    where T: FuturesType,
{
    fn get_api(&self, f: FuturesRoute)-> String{
        (self.router)(f)
    }

    /// Get a listen key for the stream
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, userstream::*, config::*};
    /// let userstream: UserStream = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(userstream.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// assert!(start.unwrap().listen_key.len() > 0)
    /// ```
    async fn start(&self) -> Result<UserDataStream> { self.client.post(self.get_api(FuturesRoute::UserDataStream).as_str(), None).await }

    /// Keep the connection alive, as the listen key becomes invalid after 60mn
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, userstream::*, config::*};
    /// let userstream: UserStream = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(userstream.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// let keep_alive = tokio_test::block_on(userstream.keep_alive(&start.unwrap().listen_key));
    /// assert!(keep_alive.is_ok())
    /// ```
    async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(self.get_api(FuturesRoute::UserDataStream).as_str(), listen_key, None).await
    }

    /// Invalidate the listen key
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, userstream::*, config::*};
    /// let userstream: UserStream = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(userstream.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// let close = tokio_test::block_on(userstream.close(&start.unwrap().listen_key));
    /// assert!(close.is_ok())
    /// ```
    async fn close(&self, listen_key: &str) -> Result<Success> {
        self.client.delete(self.get_api(FuturesRoute::UserDataStream).as_str(), listen_key, None).await
    }
}
