YEAR ?= 2023
DAY ?=

SHFMT_FLAGS = -i 0 -ci
SHELLCHECK_FLAGS = -x -P scripts
SCRIPTS = $(wildcard scripts/*.sh) $(wildcard .githooks/*)

pad = $(shell printf '%02d' $(DAY))

.PHONY: help hooks day run test check fmt lint clean

help:
	@echo 'make day YEAR=2023 DAY=7    scaffold a day'
	@echo 'make run YEAR=2023 DAY=7    run it (release)'
	@echo 'make test [YEAR=2023] [DAY=7] cargo test - whole year, or one day'
	@echo 'make check                  clippy + rustfmt check'
	@echo 'make fmt                    format rust + shell + markdown'
	@echo 'make lint                   shellcheck + shfmt/dprint diff'
	@echo 'make clean                  cargo clean'
	@echo 'make hooks                  enable .githooks for this clone'

hooks:
	git config core.hooksPath .githooks
	@echo 'hooks enabled - skip one commit with --no-verify'

day:
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	scripts/new-day.sh $(YEAR) $(DAY)

run:
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	cargo run --release -p aoc$(YEAR) --bin day$(pad)

test:
	cargo test -p aoc$(YEAR) $(if $(DAY),--bin day$(pad))

check:
	cargo clippy --workspace --all-targets -- -D warnings
	cargo fmt --all --check

fmt:
	cargo fmt --all
	shfmt $(SHFMT_FLAGS) -w $(SCRIPTS)
	dprint fmt

lint:
	shfmt $(SHFMT_FLAGS) -d $(SCRIPTS)
	shellcheck $(SHELLCHECK_FLAGS) $(SCRIPTS)
	dprint check

clean:
	cargo clean
