use std::sync::Arc;

use anyhow::Result;

use mmb_lib::{
    core::exchanges::common::ExchangeId,
    core::exchanges::{
        common::ExchangeAccountId,
        timeouts::requests_timeout_manager_factory::RequestsTimeoutManagerFactory,
        timeouts::timeout_manager::TimeoutManager,
    },
    core::lifecycle::launcher::EngineBuildConfig,
    hashmap,
};

pub(crate) fn get_binance_credentials() -> Result<(String, String)> {
    let api_key = std::env::var("BINANCE_API_KEY");
    let api_key = match api_key {
        Ok(v) => v,
        Err(_) => {
            return Err(anyhow::Error::msg(
                "Environment variable BINANCE_API_KEY are not set. Unable to continue test",
            ));
        }
    };

    let secret_key = std::env::var("BINANCE_SECRET_KEY");
    let secret_key = match secret_key {
        Ok(v) => v,
        Err(_) => {
            return Err(anyhow::Error::msg(
                "Environment variable BINANCE_SECRET_KEY are not set. Unable to continue test",
            ));
        }
    };

    Ok((api_key, secret_key))
}

// Get data to access binance account
#[macro_export]
macro_rules! get_binance_credentials_or_exit {
    () => {{
        match crate::binance::common::get_binance_credentials() {
            Ok((api_key, secret_key)) => (api_key, secret_key),
            Err(error) => {
                dbg!("{:?}", error);
                return;
            }
        }
    }};
}

pub(crate) fn get_timeout_manager(exchange_account_id: &ExchangeAccountId) -> Arc<TimeoutManager> {
    let engine_build_config = EngineBuildConfig::standard();
    let timeout_arguments = engine_build_config.supported_exchange_clients
        [&ExchangeId::new("Binance".into())]
        .get_timeout_argments();
    let request_timeout_manager = RequestsTimeoutManagerFactory::from_requests_per_period(
        timeout_arguments,
        exchange_account_id.clone(),
    );

    TimeoutManager::new(hashmap![exchange_account_id.clone() => request_timeout_manager])
}
