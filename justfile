

export LOG_LEVEL := "info"
export IN_MEMORY := "true"

run:
    cargo run


verify: test lint

test:
    cargo test

lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    cargo fix --allow-dirty --allow-staged
    cargo clippy --fix --bin "hodis" --allow-dirty --allow-staged
