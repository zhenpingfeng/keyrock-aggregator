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

pub struct BitstampWebSocket<const N: usize> {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    buf: VecDeque<OrderBook<10_usize>>,
}

impl<const N: usize> BitstampWebSocket<N> {
    pub async fn connect<S>(symbol: S) -> Result<Exchange<OrderBook<10>>>
    where
        S: Into<String>,
    {
        let (mut stream, _) = connect_async(format!("wss://ws.bitstamp.net",)).await?;

        let subscribe = format!(
            r#"
            {{
                "event": "bts:subscribe",
                "data": {{
                    "channel": "order_book_{}"
                }}
            }}"#,
            symbol.into()
        );
        stream.send(subscribe.into()).await?;

        let stream = Self {
            stream,
            buf: VecDeque::new(),
        }
        .boxed();

        Ok(Exchange {
            id: ExchangeId::Bitstamp,
            stream,
        })
    }

    async fn next_orderbook(&mut self) -> Result<OrderBook<10_usize>> {
        loop {
            if let Some(msg) = self.stream.next().await {
                let msg = msg?;

                match msg {
                    Message::Text(text) => {
                        if let Some(orderbook) = parse::parse_order_book(&text) {
                            return Ok(orderbook);
                        }
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

impl<const N: usize> Stream for BitstampWebSocket<N> {
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

    #[derive(Deserialize)]
    struct Message<'a> {
        #[serde(borrow)]
        event: &'a str,

        #[serde(borrow)]
        data: &'a serde_json::value::RawValue,
    }

    pub fn parse_order_book(raw: &str) -> Option<OrderBook<10>> {
        let message: Message = serde_json::from_str(raw).unwrap();
        match message.event {
            "data" => {
                let book: Book = serde_json::from_str(message.data.get()).unwrap();
                Some(OrderBook::parse(&book.bids, &book.asks))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_book_message() {
        let message = include_str!("../../test_data/bitstamp_book.json");
        let book = parse::parse_order_book(message).unwrap();

        assert_eq!(book.bid[0], (0.07469407, 0.05000000).into());
        assert_eq!(book.bid[2], (0.07468831, 0.46500000).into());

        assert_eq!(book.ask[0], (0.07473221, 1.61774202).into());
        assert_eq!(book.ask[2], (0.07473638, 16.18133971).into());
    }
}
