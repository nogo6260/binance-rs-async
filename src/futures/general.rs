use crate::client::*;
use crate::errors::*;
use crate::futures::rest_model::*;
use crate::rest_model::{ServerTime, Success};
use super::router::*;

pub struct FuturesGeneral {
    pub client: Client,
    pub router: fn(Futures) -> String,
}

impl FuturesGeneral {

    // Test connectivity
    pub async fn ping(&self) -> Result<Success> {
        self.client.get((self.router)(Futures::Ping).as_str(), None).await
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get_p((self.router)(Futures::Time).as_str(), None).await
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get_p((self.router)(Futures::ExchangeInfo).as_str(), None).await
    }

    // Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let symbol_string = symbol.into();
        let upper_symbol = symbol_string.to_uppercase();
        match self.exchange_info().await {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                Err(Error::UnknownSymbol(symbol_string.clone()))
            }
            Err(e) => Err(e),
        }
    }
}
