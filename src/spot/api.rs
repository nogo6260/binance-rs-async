
use super::account::Account;
use super::margin::Margin;
use super::market::Market;
use super::trade::Trade;

pub struct Spot {
    pub account: Account,
    pub margin: Margin,
    pub market: Market,
    pub trade: Trade,
}
