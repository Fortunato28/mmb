#[cfg(test)]
use std::{collections::HashMap, sync::Arc};

use crate::core::balance_manager::balance_request::BalanceRequest;
use crate::core::balances::virtual_balance_holder::VirtualBalanceHolder;
use crate::core::exchanges::common::{CurrencyCode, CurrencyPair};
use crate::core::exchanges::general::currency_pair_metadata::CurrencyPairMetadata;
use crate::core::exchanges::general::test_helper::get_test_exchange_by_currency_codes_and_amount_code;
use crate::core::exchanges::{
    common::ExchangeAccountId,
    general::{exchange::Exchange, test_helper::get_test_exchange_by_currency_codes},
};
use crate::core::service_configuration::configuration_descriptor::ConfigurationDescriptor;

struct VirtualBalanceHolderTests {
    virtual_balance_holder: VirtualBalanceHolder,
    pub exchange_account_id: ExchangeAccountId,
    exchange: Arc<Exchange>,
    pub currency_pair_metadata: Arc<CurrencyPairMetadata>,
    configuration_descriptor: Arc<ConfigurationDescriptor>,
}

impl VirtualBalanceHolderTests {
    pub fn new() -> Self {
        let tmp_exchange = get_test_exchange_by_currency_codes(
            false,
            VirtualBalanceHolderTests::eth().as_str(),
            VirtualBalanceHolderTests::btc().as_str(),
        )
        .0;
        VirtualBalanceHolderTests::new_core(tmp_exchange)
    }

    pub fn new_with_amount(amount_currency_code: &str) -> Self {
        let tmp_exchange = get_test_exchange_by_currency_codes_and_amount_code(
            false,
            VirtualBalanceHolderTests::eth().as_str(),
            VirtualBalanceHolderTests::btc().as_str(),
            amount_currency_code,
        )
        .0;
        VirtualBalanceHolderTests::new_core(tmp_exchange)
    }

    fn new_core(tmp_exchange: Arc<Exchange>) -> Self {
        let exchange_account_id = tmp_exchange.exchange_account_id.clone();
        let mut exchanges_by_id = HashMap::new();
        exchanges_by_id.insert(exchange_account_id.clone(), tmp_exchange.clone());

        Self {
            virtual_balance_holder: VirtualBalanceHolder::new(exchanges_by_id),
            exchange_account_id: exchange_account_id.clone(),
            exchange: tmp_exchange.clone(),
            currency_pair_metadata: tmp_exchange
                .get_currency_pair_metadata(&VirtualBalanceHolderTests::currency_pair())
                .expect("in test"),
            configuration_descriptor: Arc::from(ConfigurationDescriptor::new(
                "service".into(),
                "config".into(),
            )),
        }
    }

    fn btc() -> CurrencyCode {
        "btc".into()
    }

    fn eth() -> CurrencyCode {
        "eth".into()
    }

    fn currency_pair() -> CurrencyPair {
        CurrencyPair::from_codes(
            &VirtualBalanceHolderTests::eth(),
            &VirtualBalanceHolderTests::btc(),
        )
    }

    fn create_balance_request(&self, currency_code: CurrencyCode) -> BalanceRequest {
        BalanceRequest::new(
            self.configuration_descriptor.clone(),
            self.exchange_account_id.clone(),
            VirtualBalanceHolderTests::currency_pair(),
            currency_code,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use crate::core::balance_manager::balance_request::BalanceRequest;
    use crate::core::logger::init_logger;

    use super::VirtualBalanceHolderTests;

    fn add_balance_and_check(
        test_obj: &mut VirtualBalanceHolderTests,
        balance_request: &BalanceRequest,
        balance_to_add: Decimal,
        expect_balance: Option<Decimal>,
    ) {
        test_obj
            .virtual_balance_holder
            .add_balance(&balance_request, balance_to_add);
        assert_eq!(
            test_obj.virtual_balance_holder.get_virtual_balance(
                &balance_request,
                test_obj.currency_pair_metadata.clone(),
                None,
                &mut None,
            ),
            expect_balance
        );
    }

    #[test]
    pub fn set_balance_simple() {
        init_logger();
        let mut test_obj = VirtualBalanceHolderTests::new();

        let exchange_account_id = test_obj.exchange_account_id.clone();
        let mut balances_by_currency_code = HashMap::new();
        balances_by_currency_code.insert(VirtualBalanceHolderTests::btc(), dec!(0));
        test_obj
            .virtual_balance_holder
            .update_balances(&exchange_account_id, &balances_by_currency_code);

        let balance_request = test_obj.create_balance_request(VirtualBalanceHolderTests::btc());
        add_balance_and_check(&mut test_obj, &balance_request, dec!(0), Some(dec!(0)));
        add_balance_and_check(&mut test_obj, &balance_request, dec!(10), Some(dec!(10)));
        add_balance_and_check(&mut test_obj, &balance_request, dec!(10), Some(dec!(20)));
    }

    #[test]
    pub fn get_exchange_balance_multiple_currency_code() {
        init_logger();
        let mut test_obj = VirtualBalanceHolderTests::new();

        let exchange_account_id = test_obj.exchange_account_id.clone();
        let mut balances_by_currency_code = HashMap::new();
        balances_by_currency_code.insert(VirtualBalanceHolderTests::btc(), dec!(0));
        balances_by_currency_code.insert(VirtualBalanceHolderTests::eth(), dec!(0));
        test_obj
            .virtual_balance_holder
            .update_balances(&exchange_account_id, &balances_by_currency_code);

        let balance_request = test_obj.create_balance_request(VirtualBalanceHolderTests::btc());
        add_balance_and_check(&mut test_obj, &balance_request, dec!(10), Some(dec!(10)));

        let balance_request = test_obj.create_balance_request(VirtualBalanceHolderTests::eth());
        add_balance_and_check(&mut test_obj, &balance_request, dec!(10), Some(dec!(10)));
    }

    #[test]
    pub fn get_all_balances_valid() {
        init_logger();
        let mut test_obj = VirtualBalanceHolderTests::new();

        let exchange_account_id = test_obj.exchange_account_id.clone();
        let mut balances_by_currency_code = HashMap::new();
        balances_by_currency_code.insert(VirtualBalanceHolderTests::btc(), dec!(50));
        test_obj
            .virtual_balance_holder
            .update_balances(&exchange_account_id, &balances_by_currency_code);

        let balance_request = test_obj.create_balance_request(VirtualBalanceHolderTests::btc());
        add_balance_and_check(&mut test_obj, &balance_request, dec!(-40), Some(dec!(10)));

        let balance_diffs = test_obj.virtual_balance_holder.get_virtual_balance_diffs();
        assert_eq!(
            balance_diffs.get_by_balance_request(&balance_request),
            Some(dec!(-40))
        );

        assert_eq!(
            test_obj.virtual_balance_holder.get_exchange_balance(
                &test_obj.exchange_account_id,
                test_obj.currency_pair_metadata,
                &VirtualBalanceHolderTests::btc(),
                None
            ),
            Some(dec!(50))
        );
    }

    #[test]
    #[ignore] // Work in progress due to derivatives
    pub fn get_balance_for_derivative_with_mark_price() {
        init_logger();
        let mut test_obj =
            VirtualBalanceHolderTests::new_with_amount(VirtualBalanceHolderTests::btc().as_str());

        let exchange_account_id = test_obj.exchange_account_id.clone();
        let mut balances_by_currency_code = HashMap::new();
        balances_by_currency_code.insert(VirtualBalanceHolderTests::eth(), dec!(50));
        test_obj
            .virtual_balance_holder
            .update_balances(&exchange_account_id, &balances_by_currency_code);

        let balance_request_btc = test_obj.create_balance_request(VirtualBalanceHolderTests::btc());
        let balance_request_eth = test_obj.create_balance_request(VirtualBalanceHolderTests::eth());
        add_balance_and_check(&mut test_obj, &balance_request_eth, dec!(0), Some(dec!(50)));
        add_balance_and_check(&mut test_obj, &balance_request_btc, dec!(0), None);

        // TODO: fix me
        // _virtualBalanceHolder.SetMarkPrice(ExchangeName, CurrencyCodePair, 2.5m);
        // _virtualBalanceHolder.GetVirtualBalance(CreateBalanceRequest(Eth), _symbol).Should().Be(50m);
        // _virtualBalanceHolder.GetVirtualBalance(CreateBalanceRequest(Btc), _symbol).Should().Be(50m * 2.5m);
    }
}
