use futures::{
    ready,
    task::{Context, Poll},
    Future, SinkExt, Stream, StreamExt,
};

use std::collections::VecDeque;
use std::pin::Pin;
use tokio::pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::error::Result;
use crate::{
    exchange::{Exchange, ExchangeId},
    orderbook::OrderBook,
};

pub struct BinanceWebSocket<const N: usize> {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    buf: VecDeque<OrderBook<10_usize>>,
}

impl<const N: usize> BinanceWebSocket<N> {
    pub async fn connect<S>(symbol: S) -> Result<Exchange<OrderBook<10>>>
    where
        S: Into<String>,
    {
        let (stream, _) = connect_async(format!(
            "wss://stream.binance.com:9443/ws/{}@depth20@100ms",
            symbol.into()
        ))
        .await?;

        let stream = Self {
            stream,
            buf: VecDeque::new(),
        }
        .boxed();

        Ok(Exchange {
            id: ExchangeId::Binance,
            stream,
        })
    }

    async fn next_orderbook(&mut self) -> Result<OrderBook<10_usize>> {
        loop {
            if let Some(msg) = self.stream.next().await {
                let msg = msg?;
                match msg {
                    Message::Text(text) => {
                        let orderbook = parse::parse_order_book(&text);
                        return Ok(orderbook);
                    }
                    Message::Ping(_) => {
                        self.stream.send(Message::Pong(b"".to_vec())).await?;
                    }
                    _ => {}
                }
            }
        }
    }
}

impl<const N: usize> Stream for BinanceWebSocket<N> {
    type Item = OrderBook<10_usize>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(data) = self.buf.pop_front() {
                return Poll::Ready(Some(data));
            }
            let orderbook = {
                let next_orderbook = self.next_orderbook();
                pin!(next_orderbook);
                match ready!(next_orderbook.poll(cx)) {
                    Ok(orderbook) => orderbook,
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            };
            self.buf.push_back(orderbook);
        }
    }
}

pub mod parse {
    use crate::orderbook::OrderBook;
    use crate::ws::utils::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Book<'a> {
        #[serde(borrow)]
        #[serde(deserialize_with = "deserialize_first_10")]
        bids: [[&'a str; 2]; 10],

        #[serde(borrow)]
        #[serde(deserialize_with = "deserialize_first_10")]
        asks: [[&'a str; 2]; 10],
    }

    pub fn parse_order_book(raw: &str) -> OrderBook<10> {
        let book: Book = serde_json::from_str(raw).unwrap();
        OrderBook::parse(&book.bids, &book.asks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_depth20_message() {
        let message = include_str!("../../test_data/binance_book.json");
        let book = parse::parse_order_book(message);

        assert_eq!(book.bid[0], (0.07500200, 17.24140000).into());
        assert_eq!(book.bid[5], (0.07498800, 6.35150000).into());

        assert_eq!(book.ask[0], (0.07500300, 10.85240000).into());
        assert_eq!(book.ask[5], (0.07502100, 7.16630000).into());
    }
}
