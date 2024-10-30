build:
	clear
	maturin develop

test:
	clear
	pytest -v

build_test:
	make build
	make test

build_run:
	make build
	clear
	python3 Main.py

