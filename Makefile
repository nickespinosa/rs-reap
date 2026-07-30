# Local automation for fmt / lint / test. CI runs the same verify path.

.PHONY: help clean fmt fmt-check lint test docs build verify ci

# rustup installs here; non-interactive make often lacks it on PATH.
export PATH := $(HOME)/.cargo/bin:$(PATH)

# macOS Make 3.81 execs simple recipes without a shell and ignores the
# Makefile PATH export above — resolve an absolute cargo once instead.
CARGO := $(shell PATH="$(HOME)/.cargo/bin:$$PATH" command -v cargo 2>/dev/null)
ifeq ($(CARGO),)
CARGO := cargo
endif

# Optional extra flags, e.g. `make test FLAGS='-- --nocapture'`.
FLAGS ?=

help:
	@echo "targets:"
	@echo "  clean       cargo clean"
	@echo "  fmt         cargo fmt --all   (do not pass --all to make)"
	@echo "  fmt-check   cargo fmt-check"
	@echo "  lint        cargo lint"
	@echo "  test        cargo test"
	@echo "  docs        cargo docs"
	@echo "  build       cargo build"
	@echo "  verify      fmt-check + lint + test"
	@echo "  ci          verify + docs (matches CI)"
	@echo ""
	@echo "note: Make parses flags before targets, so \`make fmt --all\` fails."
	@echo "      Use \`make fmt\` — it already runs cargo fmt --all."

clean:
	$(CARGO) clean $(FLAGS)

fmt:
	$(CARGO) fmt --all $(FLAGS)

fmt-check:
	$(CARGO) fmt-check $(FLAGS)

lint:
	$(CARGO) lint $(FLAGS)

test:
	$(CARGO) test $(FLAGS)

docs:
	$(CARGO) docs $(FLAGS)

build:
	$(CARGO) build $(FLAGS)

verify: fmt-check lint test

ci: verify docs
