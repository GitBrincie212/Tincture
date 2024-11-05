build:
	clear
	maturin develop

prod:
	clear
	cargo build --profile release
	mv target/release/libtincture.dylib ./tincture.so

test:
	clear
	pytest -v

strict_test:
	clear
	cargo clippy
	cargo fmt
	tox -p
	rm -r ./.tox

build_strict_test:
	clear
	make build
	make prod_test

build_test:
	make build
	make test

