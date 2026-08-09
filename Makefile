YEAR ?= 2023
DAY ?=

SHFMT_FLAGS = -i 0 -ci
SHELLCHECK_FLAGS = -x -P scripts
SCRIPTS = $(wildcard scripts/*.sh) $(wildcard .githooks/*)

pad = $(shell printf '%02d' $(DAY))

# scope: nothing = whole workspace, YEAR = one year, YEAR + DAY = one day.
# YEAR only narrows when passed explicitly - the ?= default above must not count.
year_set = $(filter command line environment,$(origin YEAR))
scope = $(if $(DAY),-p aoc$(YEAR) --bin day$(pad),$(if $(year_set),-p aoc$(YEAR),--workspace))
# --all-targets and --tests both override --bin rather than narrowing it. For one day,
# --profile test is what compiles the bin together with its own #[cfg(test)] module.
lint_scope = $(scope) $(if $(DAY),--profile test,--all-targets)
fmt_scope = $(if $(year_set)$(DAY),-p aoc$(YEAR),--all)

.PHONY: help hooks day run test check pedantic fmt lint clean

help:
	@echo 'scope: no YEAR = whole workspace, YEAR = one year, YEAR + DAY = one day'
	@echo
	@echo 'make day YEAR=2023 DAY=7        scaffold a day'
	@echo 'make run YEAR=2023 DAY=7        run it (release) - DAY required'
	@echo 'make test [YEAR=..] [DAY=..]    cargo test'
	@echo 'make check [YEAR=..] [DAY=..]   clippy -D warnings + rustfmt check'
	@echo 'make pedantic [YEAR=..] [DAY=..] clippy::pedantic - advisory'
	@echo 'make fmt                        format rust + shell + markdown'
	@echo 'make lint                       shellcheck + shfmt/dprint diff'
	@echo 'make clean                      cargo clean'
	@echo 'make hooks                      enable .githooks for this clone'

hooks:
	git config core.hooksPath .githooks
	@echo 'hooks enabled - skip one commit with --no-verify'

day:
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	scripts/new-day.sh $(YEAR) $(DAY)

run:
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	cargo run --release $(scope)

test:
	cargo test $(scope)

check:
	cargo clippy $(lint_scope) -- -D warnings
	cargo fmt $(fmt_scope) --check

# advisory only - pedantic flags plenty that is fine in a puzzle solution
pedantic:
	cargo clippy $(lint_scope) -- -W clippy::pedantic

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
