.PHONY: all test

all:
	cargo run

test:
	cargo test

count:
	fd | grep "\.rs" | xargs wc -l
