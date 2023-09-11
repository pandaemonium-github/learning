5 better ways to code in rust: youtube / let's get rusty
- continuous integration


# Flamegraph

```bash
cargo install flamegraph
```

```bash
# if you'd like to profile an arbitrary executable:
flamegraph [-o my_flamegraph.svg] -- /path/to/my/binary --my-arg 5

# or if the executable is already running, you can provide the PID via `-p` (or `--pid`) flag:
flamegraph [-o my_flamegraph.svg] --pid 1337

# NOTE: By default, perf tries to compute which functions are
# inlined at every stack frame for every sample. This can take
# a very long time (see https://github.com/flamegraph-rs/flamegraph/issues/74).
# If you don't want this, you can pass --no-inline to flamegraph:
flamegraph --no-inline [-o my_flamegraph.svg] /path/to/my/binary --my-arg 5

# cargo support provided through the cargo-flamegraph binary!
# defaults to profiling cargo run --release
cargo flamegraph

# by default, `--release` profile is used,
# but you can override this:
cargo flamegraph --dev

# if you'd like to profile a specific binary:
cargo flamegraph --bin=stress2

# if you want to pass arguments as you would with cargo run:
cargo flamegraph -- my-command --my-arg my-value -m -f

# if you want to use interesting perf or dtrace options, use `-c`
# this is handy for correlating things like branch-misses, cache-misses,
# or anything else available via `perf list` or dtrace for your system
cargo flamegraph -c "record -e branch-misses -c 100 --call-graph lbr -g"

# Run criterion benchmark
# Note that the last --bench is required for `criterion 0.3` to run in benchmark mode, instead of test mode.
cargo flamegraph --bench some_benchmark --features some_features -- --bench

cargo flamegraph --example some_example --features some_features

# Profile unit tests.
# Note that a separating `--` is necessary if `--unit-test` is the last flag.
cargo flamegraph --unit-test -- test::in::package::with::single::crate
cargo flamegraph --unit-test crate_name -- test::in::package::with::multiple:crate
cargo flamegraph --unit-test --dev test::may::omit::separator::if::unit::test::flag::not::last::flag

# Profile integration tests.
cargo flamegraph --test test_name
```

# Dhat
Heap allocation analyser

Heap profiling Rust programs with DHAT: https://www.youtube.com/watch?v=AJhKaoyc4pY

in Cargo.toml:
```toml
[dependencies]
dhat="0.3.2"

[profile.release] # compile in release when doing heap profiling, otherwise it's too slow
debug = 1 # but add debug information to ease analysis

[features]
dhat-heap = [] # if you are doing heap profiling
```

in main.rs:
```rust
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
  #[cfg(feature = "dhat-heap")]
  let _profiler =  dhat::Profiler::new_heap();

  println!("Hello, world!");
}
```

```bash
cargo run --feature dhat-heap
```

