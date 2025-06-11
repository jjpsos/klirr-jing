.PHONY: help, _make_and_open, ooo, service, expense

service:
	CMD="cargo run --bin invoice" $(MAKE) _make_and_open

help:
	cargo run --bin invoice -- --help

expense:
	CMD="cargo run --bin invoice -- expenses" $(MAKE) _make_and_open
	
_make_and_open:
	@rm -f output.pdf && $(CMD) && open output.pdf

# Usage: `make ooo DAYS_OFF=5`
ooo:
	@if [ -z "$(DAYS_OFF)" ]; then \
		echo "Error: DAYS_OFF is required"; \
		exit 1; \
	fi; \
	CMD="cargo run --bin invoice -- ooo $(DAYS_OFF)" $(MAKE) _make_and_open
