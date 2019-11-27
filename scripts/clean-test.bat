cargo fmt --all
cargo fix --workspace --allow-staged --allow-dirty --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --all-features -- --nocapture