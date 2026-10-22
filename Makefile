
check:
	cargo check

test:
	cargo nextest run

fmt:
	cargo +nightly fmt

docker:
	docker build .

.PHONY: fmt check
