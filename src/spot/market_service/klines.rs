use crate::{client::Client};
use crate::util::*;
use crate::errors::*;

/// K线数据
/// https://binance-docs.github.io/apidocs/spot/cn/#k
pub struct KlinesBuilder<'a> {
    client: &'a Client,
    symbol: String,
    interval: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u16>,
}

impl<'a> KlinesBuilder<'a> {
    pub fn new(client: &'a Client, symbol: String ,interval :String) -> Self {
        Self {
            client,
            symbol,
            interval: interval,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
       self.start_time = Some(start_time);
       self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
       self.end_time = Some(end_time);
       self
    }

    pub fn limit(mut self, limit: u16) -> Self {
       self.limit = Some(limit);
       self
    }
    
    fn get_params(&self) -> String {
        let parameters = IntoIterator::into_iter([
            Some(("symbol", self.symbol.to_string())),
            Some(("interval", self.interval.to_string())),
            self.start_time.map(|l| ("startTime", l.to_string())),
            self.end_time.map(|l| ("endTime", l.to_string())),
            self.limit.map(|l| ("limit", l.to_string())),
        ])
        .flatten();
        build_request(parameters)
    }

    pub async fn send(self) -> Result<Vec<KlineSummary>> {
        let data = self.get_params();
        let resp:Vec<Vec<serde_json::Value>> = self.client.get("/api/v3/klines", Some(data.as_str())).await?;
        let klines:Vec<KlineSummary> = resp.iter().map(|row| KlineSummary {
            open_time: to_i64(&row[0]),
            open: to_f64(&row[1]),
            high: to_f64(&row[2]),
            low: to_f64(&row[3]),
            close: to_f64(&row[4]),
            volume: to_f64(&row[5]),
            close_time: to_i64(&row[6]),
            quote_asset_volume: to_f64(&row[7]),
            number_of_trades: to_i64(&row[8]),
            taker_buy_base_asset_volume: to_f64(&row[9]),
            taker_buy_quote_asset_volume: to_f64(&row[10]),
        }).collect();
        Ok(klines)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KlineSummary {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_asset_volume: f64,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}