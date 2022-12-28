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
use les::exchange::binance::model::TradeHistoryReq;

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
        environment: Default::default(),
        credentials: Some(BinanceCredentials {
            api_key: env::var("BINANCE_API_KEY").expect("Couldn't get environment variable"),
            api_secret: env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable"),
        }),
    })
    .await
    .expect("Failed to create Client")
}

#[tokio::test]
async fn trade_history() {
    let exchange = init().await;
    let params = TradeHistoryReq {
        paginator: None,
        symbol: String::from("BNBBTC"),
    };

    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .trade_history(&params)
        .await
        .expect("Couldn't trade history.");
    println!("{:?}", resp);
}