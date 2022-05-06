# Aggregator

Aggregator gRPC implement for keyrock challenge


## Cli Client Usage

```rust
# start grpc server
cargo run --bin aggr-server

# start grpc client connect to server
cargo run --bin aggr-client

# bench deserialize performance
cargo bench
```

## Web 3D Client Usage

>this is an fork from [3D_orderbook](https://github.com/Is0tope/3D_order_book/) and modified for this gRPC chanllenge 

```bash
cd 3D_order_book

#1 install dependencies
yarn install

#2 start envoy grpc proxy server make sure port 9090 not being used, u can change the port in enovy.yaml 
sudo docker run -d -v "$(pwd)"/envoy.yaml:/etc/envoy/envoy.yaml:ro \
    --network=host envoyproxy/envoy:v1.22.0
```
> NOTE: As per [this issue](https://github.com/grpc/grpc-web/issues/436): if
> you are running Docker on Mac/Windows, change the envoy.yaml file last `address: 0.0.0.0` to
>
> ```yaml
>     ...
>     socket_address:
>         address: host.docker.internal
> ```
>
> or if your version of Docker on Mac older then v18.03.0, change it to:
>
> ```yaml
>     ...
>     socket_address:
>         address: docker.for.mac.localhost
> ```
```
#3 run the web app
yarn serve

# then use browser open http://localhost:8081/ u should can see ui like below
```
!["web grpc client](./ob.jpg?raw=true "web grpc client")