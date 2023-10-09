
```sh
cargo add env_logger
```

```rust
use log::{debug, info};
fn main() 
  env_logger::init();
  let i = 0;
  info!("i={i}");
}
```

```sh
RUST_LOG=info cargo run
cargo watch -q -c -w /src -E RUST_LOG=debug -x run
```
