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

pub struct Exchange<T> {
    pub id: ExchangeId,
    pub stream: BoxStream<'static, T>,
}
