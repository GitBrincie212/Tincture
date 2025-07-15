build:
	clear
	maturin develop
	rm -f ./tincture.so
	mv ./python/tincture/tincture.cpython-312-darwin.so ./tincture.so

prod:
	clear
	cargo build --profile release
	rm -f ./tincture.so
	mv target/release/libtincture.dylib ./tincture.so

debug_prod:
	clear
	cargo build --profile debug-release
	rm -f ./tincture.so
	mv target/debug-release/libtincture.dylib ./tincture.so

test:
	clear
	pytest -v

