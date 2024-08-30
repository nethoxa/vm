test:
cargo test -- --nocapture

build:
cargo build --release

fmt:
cargo fmt

all:
cargo fmt && cargo build --release