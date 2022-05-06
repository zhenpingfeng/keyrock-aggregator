#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;
use aggregator::orderbook::OrderBook;
use aggregator::ws::models::{BinanceOrderBookEvent, BitstampOrderbookEvent, PriceLevel};
use aggregator::ws::{binance, bitstamp};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bn_normal_serde_json(text: &str) -> OrderBook<10> {
    let orderbook: BinanceOrderBookEvent = serde_json::from_str(text).unwrap();
    OrderBook {
        bid: orderbook.bids.iter().take(10).map(|l| *l).collect::<Vec<PriceLevel>>().try_into().unwrap(),
        ask: orderbook.asks.iter().take(10).map(|l| *l).collect::<Vec<PriceLevel>>().try_into().unwrap(),
    }
}

fn bn_custom_deserilize(text: &str) -> OrderBook<10> {
    let orderbook = binance::parse::parse_order_book(&text);
    orderbook
}

fn bts_normal_serde_json(text: &str) -> OrderBook<10> {
    let orderbook: BitstampOrderbookEvent = serde_json::from_str(text).unwrap();
    OrderBook {
        bid: orderbook.data.bids.iter().take(10).map(|l| *l).collect::<Vec<PriceLevel>>().try_into().unwrap(),
        ask: orderbook.data.asks.iter().take(10).map(|l| *l).collect::<Vec<PriceLevel>>().try_into().unwrap(),
    }
}

fn bts_custom_deserilize(text: &str) -> OrderBook<10> {
    let orderbook = bitstamp::parse::parse_order_book(&text).unwrap();
    orderbook
}

fn criterion_benchmark(c: &mut Criterion) {
    let binance = include_str!("../../test_data/binance_book.json");
    let bitstamp = include_str!("../../test_data/bitstamp_book.json");

    c.bench_function("binance normal_serde_json", |b| {
        b.iter(|| bn_normal_serde_json(black_box(binance)))
    });
    c.bench_function("binance custom_deserilize", |b| {
        b.iter(|| bn_custom_deserilize(black_box(binance)))
    });
    c.bench_function("bitstamp normal_serde_json", |b| {
        b.iter(|| bts_normal_serde_json(black_box(bitstamp)))
    });
    c.bench_function("bitstamp custom_deserilize", |b| {
        b.iter(|| bts_custom_deserilize(black_box(bitstamp)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
