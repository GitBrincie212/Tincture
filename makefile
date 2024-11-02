build:
	clear
	maturin develop

test:
	clear
	pytest -v

prod_test:
	clear
	tox -p
	rm -r ./.tox

build_prod_test:
	clear
	make build
	make prod_test

build_test:
	make build
	make test

