use dotenv::dotenv;
use futures_util::StreamExt;
use tokio::time::timeout;
use std::{env, time::Duration};
use les::{
    prelude::{*, websocket::{Subscription, WebSocketResponse, LesWebSocketMessage}, market_pair::Currency},
    model::{
        CancelAllOrdersRequest, CancelOrderRequest, GetOrderHistoryRequest, OrderRequest,
        OpenMarketOrderRequest, TimeInForce, TradeHistoryRequest, GetPriceTickerRequest
    }, exchange::{binance::{BinanceWebsocket, model::websocket::BinanceSubscription}},
};
use rust_decimal::prelude::*;
use les::exchange::binance::{Binance, BinanceCredentials, BinanceParameters};
use les::exchange::binance::model::{KlineParams, KlineSummaries, TradeHistoryReq};
use les::model::websocket::Subscription::Trades;
use les::model::websocket::WebSocketResponse::Generic;
use les::model::websocket::LesWebSocketMessage::{OrderBook, Trades as WSTrades};

async fn get_current_price(exchange: &impl Exchange, market_pair: &MarketPair, multiplier: f32) -> Decimal {
    let market_pair = market_pair.clone();
    let ticker = exchange
        .get_price_ticker(&GetPriceTickerRequest { market_pair })
        .await
        .expect("Failed to get price ticker.");
    let price = ticker.price.unwrap_or(Decimal::from_f32(1.0).unwrap());
    price * Decimal::from_f32(multiplier).unwrap()
}


async fn init() -> Binance {
    dotenv().ok();
    Binance::new(BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key: env::var("BINANCE_API_KEY").expect("Couldn't get environment variable"),
            api_secret: env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable"),
        }),
        ..Default::default()
    })
    .await
    .expect("Failed to create Client")
}


#[tokio::test]
async fn trade_stream() {
    dotenv().ok();
    let binance_ws = BinanceWebsocket::new(BinanceParameters::sandbox()).await.expect("Failed to create Client");
    // cli.create_stream_specific(vec![BinanceSubscription::Candlestick("")])


    let market = MarketPair(Currency::ETH, Currency::BTC);

    // let tx = binance_ws.subscribe(Trades(market), move |m| {
    //     let r = m.as_ref();
    //
    //     if let Ok(Generic(WSTrades(order_book))) = r {
    //         println!("{:?}", order_book)
    //     } else if let Err(err) = r {
    //         println!("{:#?}", err);
    //     }
    // })
    //     .await
    //     .expect("Failed to subscribe to orderbook on Coinbase");
    //
    // std::thread::sleep(std::time::Duration::from_millis(5000));
    let mut stream = binance_ws.create_stream(&[Subscription::Candlestick(market, "1s".to_string())])
        .await
        .expect("Couldn't create stream.");
    for _ in 0..2 {
        let message_timeout = timeout(Duration::new(200, 0), stream.next()).await;
        // let message = stream.next();
        if let Ok(message) = message_timeout {
            let message = message
                .expect("Failed to stream trades.")
                .expect("Stream error.");
            match message {
                WebSocketResponse::Raw(trades) => {
                    println!("{:#?}", trades);
                },
                _ => println!("Incorrect message: {:#?}", message)
            }
        }
    }
    //
    // for _ in 0..2 {
    //     let message_timeout = timeout(Duration::new(200, 0), stream.next()).await;
    //     if let Ok(message) = message_timeout {
    //         let message = message
    //             .expect("Failed to stream trades.")
    //             .expect("Stream error.");
    //         match message {
    //             WebSocketResponse::Generic(LesWebSocketMessage::Trades(trades)) => {
    //                 println!("{:#?}", trades);
    //             },
    //             _ => panic!("Incorrect message: {:#?}", message)
    //         }
    //     }
    // }
}

#[tokio::test]
async fn trade_history() {
    let exchange = init().await;
    let page = Paginator::default();
    // {
    //     // start_time: Some(),
    //     // end_time: None,
    //     // limit: Some(100),
    //     // after: None,
    //     ..Default::default()
    // };
    let params = TradeHistoryReq {
        paginator: None,
        symbol: String::from("BNBBTC"),
    };
    let b = exchange
        .inner_client()
        .expect("Couldn't get inner time.");

    let p = KlineParams{
        symbol: "BNBBTC".to_string(),
        interval: "1m".to_string(),
        paginator: None,
    };
    let resp = b.get_klines(&p).await.expect("Couldn't trade history.");
    eprintln!("{:#?}", resp);

    let resp = b.trade_history(&params).await.expect("Couldn't trade history.");
    println!("{:?}", resp);
}