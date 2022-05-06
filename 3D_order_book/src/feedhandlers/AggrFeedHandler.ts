import { PriceLevel } from '../L2Book'
import { FeedHandler, OrderBookEvent } from './GFeedhandler'
import { Summary } from '../../generated/service_pb';


export class AggrFeedHandler extends FeedHandler{
    constructor() {
        super('AGGR','http://127.0.0.1:9090')
    }

    onOpen(): void {
        var stream = this.grpc();
        
        stream.on('data', (summary: Summary) => {
            console.log(summary);
            this.onMessage(summary);
        })
    }

    onMessage(summary: Summary): void {
        return this.handleOrderBookEvent(summary)
    }

    handleOrderBookEvent(summary: Summary) {
        let bids: PriceLevel[] = summary.getBidsList().map(function(x){
            return [x.getPrice(), x.getAmount()];
        });
        let asks: PriceLevel[] = summary.getBidsList().map(function(x){
            return [x.getPrice(), x.getAmount()];
        });

        const event: OrderBookEvent = {
            spread: summary.getSpread(),
            bids: bids,
            asks: asks
        }
        this.publishOrderBookEvent(event)
    }
}