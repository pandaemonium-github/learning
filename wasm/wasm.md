wasm + wasix (https://wasix.org/)

spin https://github.com/fermyon/spin

## 

https://github.com/NightsWatchGames/jump-jump/blob/master/README_EN.08/06/2023

``` sh
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```

```sh
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/jump-jump.wasm32-unknown-unknown
```
