pub mod pb {
    tonic::include_proto!("orderbook");
}

use futures::StreamExt;
use pb::orderbook_aggregator_client::OrderbookAggregatorClient;

#[tokio::main]
async fn main() {
    let mut client = OrderbookAggregatorClient::connect("http://127.0.0.1:50051")
        .await
        .unwrap();

    let mut stream = client
        .book_summary(pb::Empty {})
        .await
        .unwrap()
        .into_inner();

    while let Some(item) = stream.next().await {
        let data = item.unwrap();
        println!(
            "\n\nrecived: {} {:?} {:?}",
            data.spread, data.asks, data.bids
        );
    }
}
