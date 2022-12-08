# Run Natively

```
cargo run
```

# Run in Browser

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
cd test_html
cp ../target/wasm32-unknown-unknown/debug/test-macroquad.wasm .
python3 -m http.server
```

Then go to http://localhost:8000/ in your browser.
