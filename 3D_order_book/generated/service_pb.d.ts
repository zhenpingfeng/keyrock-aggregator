// package: orderbook
// file: service.proto

import * as jspb from "google-protobuf";

export class Empty extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Empty.AsObject;
  static toObject(includeInstance: boolean, msg: Empty): Empty.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Empty, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Empty;
  static deserializeBinaryFromReader(message: Empty, reader: jspb.BinaryReader): Empty;
}

export namespace Empty {
  export type AsObject = {
  }
}

export class Summary extends jspb.Message {
  getSpread(): number;
  setSpread(value: number): void;

  clearBidsList(): void;
  getBidsList(): Array<Level>;
  setBidsList(value: Array<Level>): void;
  addBids(value?: Level, index?: number): Level;

  clearAsksList(): void;
  getAsksList(): Array<Level>;
  setAsksList(value: Array<Level>): void;
  addAsks(value?: Level, index?: number): Level;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Summary.AsObject;
  static toObject(includeInstance: boolean, msg: Summary): Summary.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Summary, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Summary;
  static deserializeBinaryFromReader(message: Summary, reader: jspb.BinaryReader): Summary;
}

export namespace Summary {
  export type AsObject = {
    spread: number,
    bidsList: Array<Level.AsObject>,
    asksList: Array<Level.AsObject>,
  }
}

export class Level extends jspb.Message {
  getExchange(): string;
  setExchange(value: string): void;

  getPrice(): number;
  setPrice(value: number): void;

  getAmount(): number;
  setAmount(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Level.AsObject;
  static toObject(includeInstance: boolean, msg: Level): Level.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Level, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Level;
  static deserializeBinaryFromReader(message: Level, reader: jspb.BinaryReader): Level;
}

export namespace Level {
  export type AsObject = {
    exchange: string,
    price: number,
    amount: number,
  }
}

