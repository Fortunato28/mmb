use mmb::exchanges::{events::AllowedEventSourceType, general::commission::Commission};
use mmb_lib::core as mmb;
use mmb_lib::core::exchanges::common::*;
use mmb_lib::core::exchanges::general::features::*;
use mmb_lib::core::lifecycle::cancellation_token::CancellationToken;
use mmb_lib::core::logger::init_logger;
use mmb_lib::core::orders::event::OrderEventType;
use rust_decimal_macros::*;

use mmb_lib::core::exchanges::events::ExchangeEvent;

use crate::binance::binance_builder::BinanceBuilder;
use crate::core::order::OrderProxy;

#[actix_rt::test]
async fn create_successfully() {
    init_logger();

    let exchange_account_id: ExchangeAccountId = "Binance0".parse().expect("in test");
    let mut binance_builder = match BinanceBuilder::try_new(
        exchange_account_id.clone(),
        CancellationToken::default(),
        ExchangeFeatures::new(
            OpenOrdersType::AllCurrencyPair,
            RestFillsFeatures::default(),
            OrderFeatures::default(),
            OrderTradeOption::default(),
            WebSocketOptions::default(),
            false,
            true,
            AllowedEventSourceType::default(),
            AllowedEventSourceType::default(),
        ),
        Commission::default(),
        true,
    )
    .await
    {
        Ok(binance_builder) => binance_builder,
        Err(_) => return,
    };

    let order_proxy = OrderProxy::new(
        exchange_account_id.clone(),
        Some("FromCreateSuccessfullyTest".to_owned()),
        CancellationToken::default(),
    );

    let order_ref = order_proxy
        .create_order(binance_builder.exchange.clone())
        .await
        .expect("Create order failed with error");

    let event = binance_builder
        .rx
        .recv()
        .await
        .expect("CreateOrderSucceeded event had to be occurred");

    let order_event = if let ExchangeEvent::OrderEvent(order_event) = event {
        order_event
    } else {
        panic!("Should receive OrderEvent")
    };

    match order_event.event_type {
        OrderEventType::CreateOrderSucceeded => {}
        _ => panic!("Should receive CreateOrderSucceeded event type"),
    }

    order_proxy
        .cancel_order_or_fail(&order_ref, binance_builder.exchange.clone())
        .await;
}

#[actix_rt::test]
async fn should_fail() {
    init_logger();

    let exchange_account_id: ExchangeAccountId = "Binance0".parse().expect("in test");
    let binance_builder = match BinanceBuilder::try_new(
        exchange_account_id.clone(),
        CancellationToken::default(),
        ExchangeFeatures::new(
            OpenOrdersType::AllCurrencyPair,
            RestFillsFeatures::default(),
            OrderFeatures::default(),
            OrderTradeOption::default(),
            WebSocketOptions::default(),
            false,
            true,
            AllowedEventSourceType::default(),
            AllowedEventSourceType::default(),
        ),
        Commission::default(),
        true,
    )
    .await
    {
        Ok(binance_builder) => binance_builder,
        Err(_) => return,
    };

    let mut order_proxy = OrderProxy::new(
        exchange_account_id.clone(),
        Some("FromShouldFailTest".to_owned()),
        CancellationToken::default(),
    );
    order_proxy.amount = dec!(1);
    order_proxy.price = dec!(0.0000000000000000001);

    match order_proxy
        .create_order(binance_builder.exchange.clone())
        .await
    {
        Ok(error) => {
            assert!(false, "Create order failed with error {:?}.", error)
        }
        Err(error) => {
            assert_eq!(
                "Exchange error: Precision is over the maximum defined for this asset.",
                error.to_string()
            );
        }
    }
}
