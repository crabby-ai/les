use dotenv::dotenv;
use std::env;
use les::{
    prelude::*,
    model::{
        CancelAllOrdersRequest, CancelOrderRequest, GetOrderHistoryRequest, OpenLimitOrderRequest,
        OpenMarketOrderRequest, TimeInForce, TradeHistoryRequest, GetPriceTickerRequest
    },
};
use rust_decimal::prelude::*;
use les::exchange::binance::{Binance, BinanceCredentials, BinanceParameters};
use les::exchange::binance::model::{KlineParams, KlineSummaries, TradeHistoryReq};

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
async fn trade_history() {
    let exchange = init().await;
    let page = Paginator {
        start_time: Some(),
        end_time: None,
        limit: Some(100),
        after: None,
        ..Default::default()
    };
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
    println!("{:?}", resp);

    let resp = b.trade_history(&params).await.expect("Couldn't trade history.");
    println!("{:?}", resp);
}