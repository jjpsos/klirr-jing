.PHONY: build

build:
	rm -f output.pdf && cargo run && open output.pdf