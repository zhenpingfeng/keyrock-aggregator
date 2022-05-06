import { FeedHandler, OrderBookEvent, OrderBookEventHandler } from './feedhandlers/GFeedhandler'
import { AggrFeedHandler } from './feedhandlers/AggrFeedHandler'

export class FeedManager {
    private _feedhandlers: Map<string,FeedHandler> = new Map()
    private _orderBookEventHandlers: OrderBookEventHandler[] = []
    
    private _exchange: string | undefined
    private _symbol: string | undefined

    constructor() {
        this.registerFeedhandler(new AggrFeedHandler())
    }

    private registerFeedhandler(fh: FeedHandler) {
        const exchange = fh.getExchange()
        if(this._feedhandlers.has(exchange)) {
            return new Error(`exchange: ${exchange} is already registered`)
        }
        // Feed manager acts as passthrough for upstream feed handlers
        fh.onOrderBookEvent((event: OrderBookEvent) => this.publishOrderBookEvent(event))
        this._feedhandlers.set(exchange,fh)
    }

    private publishOrderBookEvent(event: OrderBookEvent) {
        this._orderBookEventHandlers.forEach((fn) => fn(event))
    }

    onOrderBookEvent(fn: OrderBookEventHandler) {
        this._orderBookEventHandlers.push(fn)
    }

    disconnect() {
        if(this._exchange === undefined) return
        const fh = this._feedhandlers.get(this._exchange)
        fh?.disconnect()
    }

    connect(exchange: string, symbol: string) {
        const fh = this._feedhandlers.get(exchange)
        if(!fh) {
            throw new Error(`${exchange} does is not a managed feed`)
        }
        this._exchange = exchange
        this._symbol = symbol
        fh.connect(this._symbol)
    }
}