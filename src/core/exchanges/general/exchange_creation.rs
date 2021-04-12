use crate::core::exchanges::common::Symbol;
use crate::core::exchanges::general::exchange::Exchange;
use crate::core::lifecycle::launcher::EngineBuildConfig;
use crate::core::settings::{CurrencyPairSetting, ExchangeSettings};
use itertools::Itertools;
use log::error;
use std::sync::Arc;

pub async fn create_exchange(
    exchange_settings: &ExchangeSettings,
    build_settings: &EngineBuildConfig,
) -> Arc<Exchange> {
    let (exchange_client, features) = build_settings.supported_exchange_clients
        [&exchange_settings.exchange_account_id.exchange_id]
        .create_exchange_client(exchange_settings.clone());

    let exchange = Exchange::new(
        exchange_settings.exchange_account_id.clone(),
        exchange_settings.web_socket_host.clone(),
        vec![],
        exchange_settings.websocket_channels.clone(),
        exchange_client,
        features,
    );

    exchange.build_metadata().await;

    if let Some(currency_pairs) = &exchange_settings.currency_pairs {
        exchange.set_symbols(get_symbols(&exchange, &currency_pairs[..]))
    }

    exchange
}

pub fn get_symbols(
    exchange: &Arc<Exchange>,
    currency_pairs: &[CurrencyPairSetting],
) -> Vec<Arc<Symbol>> {
    let mut symbols = Vec::new();

    let supported_symbols_guard = exchange.supported_symbols.lock();
    for currency_pair_setting in currency_pairs {
        let mut filtered_symbols = supported_symbols_guard
            .iter()
            .filter(|x| {
                if let Some(currency_pair) = &currency_pair_setting.currency_pair {
                    return currency_pair.as_str() == x.specific_currency_pair.as_str();
                }

                return x.base_currency_code == currency_pair_setting.base
                    && x.quote_currency_code == currency_pair_setting.quote;
            })
            .take(2)
            .collect_vec();

        let symbol = match filtered_symbols.len() {
            0 => {
                error!(
                    "Unsupported symbol {:?} on exchange {}",
                    currency_pair_setting, exchange.exchange_account_id
                );
                continue;
            }
            1 => filtered_symbols
                .pop()
                .expect("we checked already that 1 symbol found"),
            _ => {
                error!(
                    "Found more then 1 symbol for currency pair {:?}. Found symbols: {:?}",
                    currency_pair_setting, filtered_symbols
                );
                continue;
            }
        };

        symbols.push(symbol.clone());
    }

    symbols
}