# Run Natively

```
cargo run --release
```

# Run in Browser

```
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
python3 -m http.server
```

Then go to http://localhost:8000/test_html in your browser.
