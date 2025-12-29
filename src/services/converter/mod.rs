use anyhow::anyhow;
use rust_decimal::Decimal;

use crate::support::generic::MoneyAmount;
use crate::support::sat_converter::SatsConverter;

const MIN: i64 =     20_000_000_000; //   2000 STH
const MAX: i64 = 2_500_000_000_000; // 25000 STH

pub struct BankerService<'p> {
    #[allow(dead_code)]
    sats_amount: i64,
    #[allow(dead_code)]
    exchange_price: &'p str,
    #[allow(dead_code)]
    expected_receive: &'p str,
    #[allow(dead_code)]
    min_receive: &'p str,
    sats: SatsConverter,
    x_exchange_price: MoneyAmount,
    x_expected_receive: MoneyAmount,
    x_min_receive: MoneyAmount,
    min: Option<i64>,
    max: Option<i64>,
}

impl<'p> BankerService<'p> {
    pub fn new(
        sats_amount: i64,
        exchange_price: &'p str,
        expected_receive: &'p str,
        min_receive: &'p str,
    ) -> anyhow::Result<Self> {
        let x_exchange_price = MoneyAmount::from_str(exchange_price).ok_or(anyhow!("Invalid exchange_price"))?;
        // Pre-calculated user awared price amount
        let x_expected_receive = MoneyAmount::from_str(expected_receive).ok_or(anyhow!("Invalid expected_receive amount"))?;
        // Pre-calculated user awared price amount
        let x_min_receive = MoneyAmount::from_str(min_receive).ok_or(anyhow!("Invalid min_receive amount"))?;

        log::debug!("exchange_price {}", x_exchange_price.to_string_fixed());
        log::debug!("expected_receive {}", x_expected_receive.to_string_fixed());
        log::debug!("min_receive {}", x_min_receive.to_string_fixed());

        Ok(Self {
            sats_amount,
            exchange_price,
            expected_receive,
            min_receive,
            sats: SatsConverter::from_sats(sats_amount),
            x_exchange_price,
            x_expected_receive,
            x_min_receive,
            min: None,
            max: None,
        })
    }

    pub fn set_range(&mut self, min: i64, max: i64) {
        self.min = Some(min);
        self.max = Some(max);
    }

    pub fn validate_range(&self) -> anyhow::Result<()> {
        if !self.sats.validate_range(self.min.unwrap_or(MIN), self.max.unwrap_or(MAX)) {
            return Err(anyhow!("Amount out of range"));
        }

        Ok(())
    }

    pub fn bid_amount(&self) -> i64 {
        self.sats.as_sats()
    }

    fn price_per_unit(&self) -> Decimal {
        self.x_exchange_price.as_decimal()
    }

    pub fn calc<F: FnOnce(i64, Decimal) -> Option<MoneyAmount>>(&self, calc: F) -> anyhow::Result<MoneyAmount> {
        calc(self.bid_amount(), self.price_per_unit())
            .ok_or(anyhow!("Calculation failed"))
    }

    pub fn get_exchange_price(&self) -> MoneyAmount {
        self.x_exchange_price
    }

    pub fn get_expected_price(&self) -> MoneyAmount {
        self.x_expected_receive
    }

    pub fn get_minimum_price(&self) -> MoneyAmount {
        self.x_min_receive
    }
}
