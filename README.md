# kuksa-sandbox
Sandbox for kuksa-databroker

## VSSメモ

- VSSでは、配列やインデックス番号(`Door[0]`)ではなく、論理名(`Door.FrontLeft`)で装備を特定する

## 参考

- [Kuksa Quickstart](https://eclipse-kuksa.github.io/kuksa-website/quickstart/)

## `kuksa-rust-sdk`

- `kuksa-rust-sdk`クレートを使うことで、Rustからkuksaを操作できる。

## 依存関係メモ

- `kuksa-rust-sdk`をビルドすると、[`protobuf-src`](https://docs.rs/protobuf-src/latest/protobuf_src/)クレートが必要になる。
- `protobuf-src`クレートはC++の`protobuf`ライブラリに依存している。
- `kuksa-rust-sdk`が`protobuf-src`に依存しているのはビルド時のみ
- `kuksa-rust-sdk`をビルドする時に、`protoc`コマンドを使うために使用している

```
$ git clone --recurse-submodules https://github.com/eclipse-kuksa/kuksa-rust-sdk.git
$ cd kuksa-rust-sdk
$ cargo build
...
error: failed to run custom build command for `protobuf-src v1.1.0+21.5`
...
  checking whether c++ supports C++11 features with -h std=c++0x... no

  --- stderr
  configure: WARNING: cannot find library containing definition of pow
  configure: error: *** A compiler with support for C++11 language features is required.

  thread 'main' panicked at /home/takeo/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/autotools-0.2.7/src/lib.rs:790:5:

  command did not execute successfully, got: exit status: 1

  build script failed, must exit now
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
$ sudo apt install libstdc++-12-dev
$ cargo build
```

- `kuksa-rust-sdk`のAPI界面に`prost_type`が染み出している
  - https://github.com/eclipse-kuksa/kuksa-rust-sdk/issues/14

- examplesに対してlddする限り、nativeのライブラリに対するdynamicリンクは行われていない

```
$ find ./target/debug/examples/ -executable -type f | xargs ldd | grep -v './target' | cut -d' ' -f1 | sort -u
	/lib64/ld-linux-x86-64.so.2
	libc.so.6
	libgcc_s.so.1
	libm.so.6
	linux-vdso.so.1
```

## examples

- databrokerを起動してからexample (`kuksa_rust_sdk`) を実行すると、`Vehicle.Speed`の値を購読して、値を取得できることを確認できるが、その後`panic`する。

```
$ docker run -it --rm --net=host ghcr.io/eclipse-kuksa/kuksa-databroker:latest --insecure
$ cargo run --example kuksa_rust_sdk
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.46s
     Running `target/debug/examples/kuksa_rust_sdk`
Successfully subscribed to "Vehicle.Speed"!
Vehicle.Speed: Datapoint { timestamp: Some(Timestamp { seconds: 1749309142, nanos: 993783081 }), value: Some(Value { typed_value: Some(Float(40.0)) }) }
Value published successful for signal "Vehicle.Speed"
Got value for Vehicle.Speed: Some(Datapoint { timestamp: Some(Timestamp { seconds: 1749309565, nanos: 320599209 }), value: Some(Value { typed_value: Some(Float(30.0)) }) })
Successfully subscribed to "Vehicle.Speed"!
Vehicle.Speed: DataEntry { path: "Vehicle.Speed", value: Some(Datapoint { timestamp: Some(Timestamp { seconds: 1749309565, nanos: 320599209 }), value: Some(Float(30.0)) }), actuator_target: None, metadata: Some(Metadata { data_type: Unspecified, entry_type: Unspecified, description: None, comment: None, deprecation: None, unit: None, value_restriction: None, entry_specific: None }) }
Successfully set datapoints
Got value for Vehicle.Speed: [DataEntry { path: "Vehicle.Speed", value: Some(Datapoint { timestamp: Some(Timestamp { seconds: 1749309565, nanos: 321886700 }), value: Some(Float(40.0)) }), actuator_target: None, metadata: Some(Metadata { data_type: Float, entry_type: Sensor, description: Some("Vehicle speed."), comment: None, deprecation: None, unit: Some("km/h"), value_restriction: None, entry_specific: None }) }]
Failed to subscribe to "Vehicle.Speed": Status(Status { code: Unimplemented, metadata: MetadataMap { headers: {"date": "Sat, 07 Jun 2025 15:19:25 GMT", "content-type": "application/grpc", "content-length": "0"} }, source: None })

thread 'main' panicked at kuksa-rust-sdk/src/sdv/databroker/v1/mod.rs:61:14:
called `Result::unwrap()` on an `Err` value: Status(Status { code: Unimplemented, metadata: MetadataMap { headers: {"date": "Sat, 07 Jun 2025 15:19:25 GMT", "content-type": "application/grpc", "content-length": "0"} }, source: None })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

## 環境構築

参考：https://github.com/eclipse-kuksa/kuksa-databroker

```
docker network create kuksa
docker run -it --rm --name Server -p 55555:55555 --network kuksa ghcr.io/eclipse-kuksa/kuksa-databroker:main --insecure
docker run -it --rm --network kuksa ghcr.io/eclipse-kuksa/kuksa-databroker-cli:main --server Server:55555
ksa.val.v1 > publish Vehicle.Speed 100.34
```