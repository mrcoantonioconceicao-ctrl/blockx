use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    BRL,
    USD,
    EUR,
    BTC,
    ETH,
    USDT,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Currency::BRL => "BRL",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::BTC => "BTC",
            Currency::ETH => "ETH",
            Currency::USDT => "USDT",
        };

        write!(f, "{value}")
    }
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_uppercase().as_str() {
            "BRL" => Ok(Currency::BRL),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "BTC" => Ok(Currency::BTC),
            "ETH" => Ok(Currency::ETH),
            "USDT" => Ok(Currency::USDT),
            _ => Err(format!("Unsupported currency: {}", value)),
        }
    }
}
