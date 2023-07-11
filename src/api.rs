use crate::account::*;
use crate::client::*;
use crate::config::Config;
use crate::futures::futures_type::FuturesType;
use crate::general::*;
use crate::market::*;
use crate::Spot;
use crate::spot::*;
use crate::userstream::*;

pub trait Binance: Sized {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    /// Create a binance API using environment variables for credentials
    /// BINANCE_API_KEY=$YOUR_API_KEY
    /// BINANCE_API_SECRET_KEY=$YOUR_SECRET_KEY
    fn new_with_env(config: &Config) -> Self {
        let api_key = std::env::var("BINANCE_API_KEY").ok();
        let secret = std::env::var("BINANCE_API_SECRET_KEY").ok();
        Self::new_with_config(api_key, secret, config)
    }

    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self;
}


impl Binance for Spot {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        let client = Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout);
        Self {
            account: account::Account { client: client.clone(), recv_window: config.recv_window },
            margin: margin::Margin { client: client.clone(), recv_window: config.recv_window },
            market: market::Market { client: client.clone(), recv_window: config.recv_window },
            trade: trade::Trade { client: client.clone(), recv_window: config.recv_window },
        }
    }
}


// old
impl Binance for General {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
        }
    }
}

impl Binance for Account {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "savings_api")]
impl Binance for crate::savings::Savings {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Market {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for UserStream {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "futures_api")]
impl<T> Binance for crate::futures::userstream::UserStream<T>
    where T: FuturesType
{
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        let host = if config.futures_rest_api_endpoint == "" {
            T::endpoint()
        } else {
            config.futures_rest_api_endpoint.clone()
        };

        Self {
            client: Client::new(api_key, secret_key, host, config.timeout),
            recv_window: config.recv_window,
            router: T::router(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "futures_api")]
impl<T> Binance for crate::futures::general::FuturesGeneral<T>
    where T: FuturesType
{
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        let host = if config.futures_rest_api_endpoint == "" {
            T::endpoint()
        } else {
            config.futures_rest_api_endpoint.clone()
        };

        Self {
            client: Client::new(
                api_key,
                secret_key,
                host,
                config.timeout,
            ),
            router: T::router(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "futures_api")]
impl<T> Binance for crate::futures::market::FuturesMarket<T>
    where T: FuturesType
{
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        let host = if config.futures_rest_api_endpoint == "" {
            T::endpoint()
        } else {
            config.futures_rest_api_endpoint.clone()
        };
        Self {
            client: Client::new(
                api_key,
                secret_key,
                host,
                config.timeout,
            ),
            recv_window: config.recv_window,
            router: T::router(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "futures_api")]
impl<T> Binance for crate::futures::account::FuturesAccount<T>
    where T: FuturesType
{
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        let host = if config.futures_rest_api_endpoint == "" {
            T::endpoint()
        } else {
            config.futures_rest_api_endpoint.clone()
        };

        Self {
            client: Client::new(
                api_key,
                secret_key,
                host,
                config.timeout,
            ),
            recv_window: config.recv_window,
            router: T::router(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "margin_api")]
impl Binance for crate::margin::Margin {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "wallet_api")]
impl Binance for crate::wallet::Wallet {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone(), config.timeout),
            recv_window: config.recv_window,
            binance_us_api: config.binance_us_api,
        }
    }
}
