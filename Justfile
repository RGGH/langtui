binary := "langtui"

build:
    cargo build --release

install: build
    cp target/release/{{binary}} ~/.cargo/bin/{{binary}}

run: install
    {{binary}}
