use crate::futures::router::{FuturesRoute, Router};

pub trait FuturesType {
    fn router() ->  fn(FuturesRoute) ->String;
    fn endpoint() -> String;
    fn ws_endpoint() -> String;
}

pub struct FuturesLinearType;

impl FuturesType for FuturesLinearType {
    fn router() -> fn(FuturesRoute) ->String {
        |f: FuturesRoute| String::from(Router::Linear(f))
    }

    fn endpoint() -> String {
        "https://fapi.binance.com".into()
    }

    fn ws_endpoint() -> String {
        "wss://fstream.binance.com".into()
    }
}

pub struct FuturesInverseType;

impl FuturesType for FuturesInverseType {
    fn router() -> fn(FuturesRoute) ->String {
        |f: FuturesRoute| String::from(Router::Inverse(f))
    }

    fn endpoint() -> String {
        "https://dapi.binance.com".into()
    }

    fn ws_endpoint() -> String {
        "wss://dstream.binance.com".into()
    }
}