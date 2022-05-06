use futures::stream::BoxStream;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExchangeId {
    Binance,
    Bitstamp,

    Aggregate,
}

impl fmt::Display for ExchangeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ExchangeId::Binance => "binance",
            ExchangeId::Bitstamp => "bitstamp",
            _ => panic!("unexpected id"),
        })
    }
}

#[derive(Clone)]
pub struct MarketId(pub String);

impl std::borrow::Borrow<str> for MarketId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

pub struct Exchange<T> {
    pub id: ExchangeId,
    pub stream: BoxStream<'static, T>,
}
