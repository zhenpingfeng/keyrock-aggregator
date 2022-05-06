mod pb {
    tonic::include_proto!("orderbook");
}
mod exchange;
mod orderbook;
mod ws;

use exchange::{Exchange, ExchangeId};
use futures::{future, stream::BoxStream, StreamExt};
use orderbook::{OrderBook, Sources};
use pb::{
    orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer},
    Empty, Summary,
};
use tonic::transport::Server;
use ws::models::PriceLevel;

fn map_to_levels<'a, I>(levels: I) -> Vec<pb::Level>
where
    I: Iterator<Item = (&'a PriceLevel, ExchangeId)>,
{
    levels
        .map(|(level, exchange_id)| pb::Level {
            exchange: exchange_id.to_string(),
            price: level.price,
            amount: level.amount,
        })
        .collect()
}

fn map_to_summary((book, sources): (OrderBook<10>, Sources<10>)) -> Summary {
    let spread = book.ask[0].price - book.bid[0].price;

    Summary {
        spread,
        bids: map_to_levels(book.bid.iter().zip(sources.bid.into_iter())),
        asks: map_to_levels(book.ask.iter().zip(sources.ask.into_iter())),
    }
}

pub fn build(exchanges: Vec<Exchange<OrderBook<10>>>) -> Exchange<(OrderBook<10>, Sources<10>)> {
    let mut aggregator = orderbook::Aggregator::new(exchanges.iter().map(|e| e.id));

    let (_conns, streams): (Vec<_>, Vec<_>) = exchanges
        .into_iter()
        .enumerate()
        .map(|(i, e)| (i, e.stream.map(move |book| (i, book))))
        .unzip();

    let stream = futures::stream::select_all(streams)
        .map(move |(i, book)| {
            aggregator.update(i, book);
            aggregator.aggregate()
        })
        .boxed();

    Exchange {
        id: ExchangeId::Aggregate,
        stream,
    }
}

pub struct Aggregator;

#[tonic::async_trait]
impl OrderbookAggregator for Aggregator {
    type BookSummaryStream = BoxStream<'static, Result<Summary, tonic::Status>>;

    async fn book_summary(
        &self,
        _request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Self::BookSummaryStream>, tonic::Status> {
        let symbol = "ethbtc";
        let binance = ws::binance::BinanceWebSocket::<10_usize>::connect(symbol);
        let bitstamp = ws::bitstamp::BitstampWebSocket::<10_usize>::connect(symbol);

        let (binance, bitstamp) = future::try_join(binance, bitstamp)
            .await
            .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let agg = build(vec![binance, bitstamp]);
        let stream = agg.stream.map(map_to_summary).map(Ok);

        Ok(tonic::Response::new(stream.boxed()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();

    println!("OrderbookAggregatorServer listening on {}", addr);

    Server::builder()
        .add_service(OrderbookAggregatorServer::new(Aggregator))
        .serve(addr)
        .await?;

    Ok(())
}
