# Rust MDD

### Build
```bash
cargo build --release
```

### Test
```bash
cargo test -- --nocapture
```

### Benchmark
Install nightly build
```bash
rustup install nightly
```
Then run the benchmark with the following command:
```bash
 cargo +nightly bench 
 ```