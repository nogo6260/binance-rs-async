use crate::{client::Client, util::build_request};
use crate::errors::*;
use crate::rest_model::*;

pub struct DepthBuilder<'a > {
    client: &'a Client,
    symbol: String,
    limit: Option<u16>,
}

impl<'a> DepthBuilder<'a>{
    pub(crate) fn new(client: &'a Client, symbol: String) -> Self {
        Self {
            client,
            symbol,
            limit: None,
        }
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    fn get_params(& self) -> String {
        let parameters = IntoIterator::into_iter([
            Some(("symbol", self.symbol.to_string())),
            self.limit.map(|l| ("limit", l.to_string())),
        ]).flatten();
        build_request(parameters)
    }

    pub async fn send(self) -> Result<OrderBook> {
        let data = self.get_params();
        self.client.get("/api/v3/depth", Some(data.as_str())).await
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}