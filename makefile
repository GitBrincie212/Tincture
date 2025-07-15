build:
	clear
	maturin develop
	rm -f ./tincture.so
	mv ./python/tincture/tincture.cpython-312-darwin.so ./tincture.so

prod:
	clear
	cargo build --profile release
	mv target/release/libtincture.dylib ./tincture.so

test:
	clear
	pytest -v

