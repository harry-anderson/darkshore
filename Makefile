.PHONY: build
build:
	cargo lambda build --release --arm64
deploy: build
	sam deploy
