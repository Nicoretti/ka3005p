default: fix test

fix:
    cargo fix --allow-dirty
    cargo fmt

test:
    cargo test --doc
    cargo test --all-targets

datasheets:
    curl https://cdn-reichelt.de/documents/datenblatt/D400/RND_320-KA3000.pdf -o RND_320-KA3000-User-Manual.pdf
    curl https://cdn-reichelt.de/documents/datenblatt/D400/RND_320-KAXXXX_COMMANDS.pdf -o RND_320-KA3000-COMMANDS.pdf

docs-build:
    mdbook build doc

docs-serve:
    mdbook serve --open doc

