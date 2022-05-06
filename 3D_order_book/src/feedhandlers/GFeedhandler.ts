import { OrderBookAction, PriceLevel, Side } from '../L2Book'
import {grpc} from "@improbable-eng/grpc-web";
import {OrderbookAggregatorClient, ResponseStream} from "../../generated/service_pb_service";
import {Empty, Summary} from "../../generated/service_pb";

export type OrderBookEventHandler = (event: OrderBookEvent) => void

export interface BaseFeedHandler {
    onOrderBookEvent: (fn: OrderBookEventHandler) => void
    connect: (symbol: string) => void
    disconnect: () => void
}

export interface OrderBookEvent {
    spread: number,
    bids: PriceLevel[],
    asks: PriceLevel[]
}

export class FeedHandler implements BaseFeedHandler{
    private _exchange: string
    private _symbol: string
    private _grpc: OrderbookAggregatorClient | undefined
    private _url: string
    private _connected: boolean
    private _orderBookEventHandlers: OrderBookEventHandler[] = []

    constructor(exchange: string, grpcUrl: string) {
        this._url = grpcUrl
        this._exchange = exchange
        this._connected = false
        this._symbol = ''
    }

    protected grpc(): ResponseStream<Summary> {
        if (this._grpc === undefined) {
            throw new Error('gRPC client must be first initiated using connect()')
        }
        return this._grpc.bookSummary(new Empty)
    }

    // @override
    onOpen() {
        throw Error('Not yet implemented')
    }

    // @override
    onMessage(summary: Summary) {
        throw Error('Not yet implemented')
    }

    onOrderBookEvent(fn: OrderBookEventHandler) {
        this._orderBookEventHandlers.push(fn)
    }

    getExchange(): string {
        return this._exchange
    }

    getSymbol(): string {
        return this._symbol
    }

    connect(symbol: string) {
        console.log(`Connecting to ${this._exchange} WebSocket feed`)
        this._grpc = new OrderbookAggregatorClient(this._url);
        this._symbol = symbol
        this.onOpen()
        this._connected = true
    }

    disconnect() {
        if(this._connected === false) return
        console.log(`Closing gRPC connection to ${this._exchange}`)
        this.grpc()
        this._grpc = undefined
        this._connected = false
    }

    protected publishOrderBookEvent(event: OrderBookEvent) {
        this._orderBookEventHandlers.forEach((fn) => fn(event))
    }
}
