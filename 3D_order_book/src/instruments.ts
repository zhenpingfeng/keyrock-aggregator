export interface Instrument {
    symbol: string
    tickSize: number
    symbolIndex?: number
}

export class InstrumentRepository {
    private _lookup: Map<string,Instrument[]> = new Map()

    constructor() {
        // This should really be done dynamically on initialisation by the front end. However, due to certain exchanges having
        // a CORS policy on their instruments endpoint, a proxy would be required, and I felt this would have added more
        // complexity to a simple demo. 
        this._lookup.set('AGGR',[
            {'symbol': 'ETHBTC', 'tickSize': 0.000001},
        ])
    }

    getExchanges(): string[] {
        return [...this._lookup.keys()]
    }

    getExchangeInstruments(exchange: string): Instrument[] {
        return this._lookup.get(exchange)!
    }

    getExchangeInstrument(exchange: string, symbol: string): Instrument {
        const symbols = this.getExchangeInstruments(exchange)
        return symbols.find((ins: Instrument) => ins.symbol === symbol)!
    }
}