.PHONY: build
deploy: build
	sam deploy
build:
	cargo lambda build --release --arm64
