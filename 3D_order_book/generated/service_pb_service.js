// package: orderbook
// file: service.proto

var service_pb = require("./service_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var OrderbookAggregator = (function () {
  function OrderbookAggregator() {}
  OrderbookAggregator.serviceName = "orderbook.OrderbookAggregator";
  return OrderbookAggregator;
}());

OrderbookAggregator.BookSummary = {
  methodName: "BookSummary",
  service: OrderbookAggregator,
  requestStream: false,
  responseStream: true,
  requestType: service_pb.Empty,
  responseType: service_pb.Summary
};

exports.OrderbookAggregator = OrderbookAggregator;

function OrderbookAggregatorClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

OrderbookAggregatorClient.prototype.bookSummary = function bookSummary(requestMessage, metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.invoke(OrderbookAggregator.BookSummary, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onMessage: function (responseMessage) {
      listeners.data.forEach(function (handler) {
        handler(responseMessage);
      });
    },
    onEnd: function (status, statusMessage, trailers) {
      listeners.status.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners.end.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners = null;
    }
  });
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

exports.OrderbookAggregatorClient = OrderbookAggregatorClient;

