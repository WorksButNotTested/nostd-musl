default: fix pretty build clippy doc build

build:
  cargo build

pretty:
  cargo fmt
  taplo fmt

fix:
    cargo fix --allow-dirty --allow-staged

clippy:
  cargo clippy

doc:
  cargo doc

