# Local automation for fmt / lint / test. CI runs the same verify path.

.PHONY: help fmt fmt-check lint test docs build verify ci

help:
	@echo "targets:"
	@echo "  fmt         cargo fmt --all"
	@echo "  fmt-check   cargo fmt-check"
	@echo "  lint        cargo lint"
	@echo "  test        cargo test"
	@echo "  docs        cargo docs"
	@echo "  build       cargo build"
	@echo "  verify      fmt-check + lint + test"
	@echo "  ci          verify + docs (matches CI)"

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt-check

lint:
	cargo lint

test:
	cargo test

docs:
	cargo docs

build:
	cargo build

verify: fmt-check lint test

ci: verify docs
