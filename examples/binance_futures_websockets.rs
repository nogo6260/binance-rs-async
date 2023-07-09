#[cfg(feature = "futures_api")]

#[macro_use]
extern crate tokio;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;

use futures::future::BoxFuture;
use futures::stream::StreamExt;
use serde_json::from_str;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;


use binance::futures::websockets::*;

#[tokio::main]
async fn main() {
    let (logger_tx, mut logger_rx) = tokio::sync::mpsc::unbounded_channel::<FuturesWebsocketEvent>();
    let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();

    let wait_loop = tokio::spawn(async move {
        'hello: loop {
            tokio::select! {
                event = logger_rx.recv() => println!("{event:?}"),
                _ = close_rx.recv() => break 'hello
            }
        }
    });
    // private api
    //user_stream().await;
    //user_stream_websocket().await;
    // public api
    let streams: Vec<BoxFuture<'static, ()>> = vec![
        Box::pin(market_websocket(logger_tx.clone())),
    ];

    for stream in streams {
        tokio::spawn(stream);
    }

    tokio::select! {
        _ = wait_loop => { println!("Finished!") }
        _ = tokio::signal::ctrl_c() => {
            println!("Closing websocket stream...");
            close_tx.send(true).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

#[allow(dead_code)]
async fn market_websocket(logger_tx: UnboundedSender<FuturesWebsocketEvent>) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trades: String = agg_trade_stream("ethusdt");
    let ticker: String = trade_stream("btcusdt");
    let mut web_socket: FuturesWebSockets = FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
        logger_tx.send(event.clone()).unwrap();
        match event {
            FuturesWebsocketEvent::AggTrade(trade) => {
                println!("Symbol: {}, price: {}, qty: {}", trade.symbol, trade.price, trade.qty);
            }
            FuturesWebsocketEvent::Trade(trade) => {
                println!("Symbol: {}, price: {}, qty: {}", trade.symbol, trade.price, trade.qty);
            }
            FuturesWebsocketEvent::DayMiniTicker(ticker) => {
                println!("Symbol: {}, price: {}, qty: {}", ticker.symbol,ticker.current_close,ticker.quote_volume);
            }
            _ => {}
        };

        Ok(())
    });

    web_socket.connect_multiple(vec![agg_trades,ticker]).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}