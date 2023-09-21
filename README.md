# ml-rust
AI and ML practical Rust project

## Training and testing data generation for NN
```bash
cargo run --bin generate_nn -- -c config/generate_nn.toml > training_data.csv
cargo run --bin generate_nn -- -c config/generate_nn.toml > testing_data.csv
```
