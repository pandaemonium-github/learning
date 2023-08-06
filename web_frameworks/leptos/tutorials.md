
## Build a full stack chatbot in Rust 
https://www.youtube.com/watch?v=vAjle3c9Xqc

## My first test
laptos with axum
https://github.com/leptos-rs/start-axum

```sh
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo leptos new --git leptos-rs/start-axum
cd <project>
cargo leptos watch
```

Compiling and deploying release

```sh
cargo leptos build --release
```
Copy start-axum from target/server/release, and site directory from target, to server.
Set following environment variables:
```sh
LEPTOS_OUTPUT_NAME="start-axum"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

and simply start-axum.

### test end-to-end

```sh
npx playwright install
cargo leptos end-to-end 
```

## add tailwindcss

npx tailwindcss -i ./input.css -o ./style/output.css