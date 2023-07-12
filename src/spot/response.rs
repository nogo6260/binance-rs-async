use rust_decimal::Decimal;

use crate::rest_model::*;
use crate::util::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i32,
    pub client_order_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub orig_qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub executed_qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    #[serde(with = "string_to_decimal")]
    pub stop_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub iceberg_qty: Decimal,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    #[serde(with = "string_to_decimal")]
    pub orig_quote_order_qty: Decimal,
}



