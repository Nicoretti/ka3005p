default: fix test

fix:
    cargo fix --allow-dirty
    cargo fmt

test:
    cargo test --doc
    cargo test --all-targets

docs-build:
    mdbook build doc

docs-serve:
    mdbook serve --open doc

