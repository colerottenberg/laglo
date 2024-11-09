use databento::{
    dbn::{pretty::Px, OhlcvMsg, Schema},
    historical::timeseries::GetRangeParams,
    HistoricalClient,
};
use polars::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use time::macros::{date, datetime};

#[derive(Debug, Clone, Default)]
struct Portfolio {
    positions: HashMap<String, f64>,
    capital: f64,
}

impl Portfolio {
    fn new(capital: f64) -> Self {
        Self {
            positions: HashMap::new(),
            capital,
        }
    }

    fn buy(&mut self, symbol: &str, amount: f64, price: f64) {
        let value = amount * price;
        self.capital -= value;
        let position = self.positions.entry(symbol.to_string()).or_insert(0.0);
        *position += amount;
    }

    fn sell(&mut self, symbol: &str, amount: f64, price: f64) {
        let value = amount * price;
        self.capital += value;
        let position = self.positions.entry(symbol.to_string()).or_insert(0.0);
        *position -= amount;
    }

    fn value(&self, prices: &HashMap<String, f64>) -> f64 {
        let mut value = self.capital;
        for (symbol, amount) in &self.positions {
            let price = prices.get(symbol).unwrap();
            value += amount * price;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
