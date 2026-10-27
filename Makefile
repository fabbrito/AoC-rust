# Recipes are bash with -uo pipefail: base.mk sets SHELL and .SHELLFLAGS.

include .config/make/base.mk
include .config/make/shell.mk
include .config/make/dprint.mk
include .config/make/rust.mk

.DEFAULT_GOAL := help # base.mk defines `deps` first

YEAR ?= 2023
DAY ?=

# printf reads 08 and 09 as bad octal, so strip a leading zero before padding.
pad = $(shell printf '%02d' $(patsubst 0%,%,$(DAY)))

# scope: nothing = whole workspace, YEAR = one year, YEAR + DAY = one day.
# YEAR only narrows when passed explicitly - the ?= default above must not count.
year_set = $(filter command line environment,$(origin YEAR))
scope = $(if $(DAY),-p aoc$(YEAR) --bin day$(pad),$(if $(year_set),-p aoc$(YEAR),--workspace))
# --all-targets and --tests both override --bin rather than narrowing it. For one day,
# --profile test is what compiles the bin together with its own #[cfg(test)] module.
lint_scope = $(scope) $(if $(DAY),--profile test,--all-targets)

# Not adopted (yet): nursery is unstable, and the other three each cost more
# ceremony than they buy in a puzzle. Advisory - see the `pedantic` target.
ADVISORY = -W clippy::nursery \
           -W clippy::arithmetic_side_effects \
           -W clippy::as_conversions \
           -W clippy::cast_precision_loss \
           -W clippy::cast_possible_truncation \
           -W clippy::cast_sign_loss

.PHONY: help day run test typecheck lint pedantic upgrade clean

define HELP_AWK
BEGIN {
	FS = ":.*##"
	printf "\nUsage: make \033[1m<target>\033[0m [YEAR=yyyy] [DAY=n]\n"
	printf "       scope: no YEAR = workspace, YEAR = one year, YEAR + DAY = one day\n"
}
/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) }
/^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2 }
endef
export HELP_AWK

##@ Help

help: ## show this help
	@awk "$$HELP_AWK" $(MAKEFILE_LIST)

##@ Solve

day: ## scaffold a day - needs YEAR and DAY
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	scripts/new-day.sh $(YEAR) $(DAY)

run: ## run one day in release - DAY required
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	cargo run --release $(scope)

test: ## cargo test - by hand, never in the gate
	cargo test $(scope)

##@ Quality

typecheck: ## cargo check - types only, no lints
	cargo check $(lint_scope)

# Clippy is not a lane: it is not fast enough to sit between you and a commit,
# so it rides here, after the gate. Clippy levels live in
# [workspace.lints.clippy]; -D warnings is what catches the rest - rustc's own
# dead_code, unused_variables and friends.
lint: check ## the gate's lanes, then clippy - read only
	cargo clippy $(lint_scope) -- -D warnings

# Advisory only - it flags plenty that is fine in a puzzle. Mine it for
# candidates after a day lands; promote a rule by moving it into Cargo.toml.
pedantic: ## the lints make lint does not deny - advisory
	cargo clippy $(lint_scope) -- $(ADVISORY)

##@ Upgrade

# mise's pins, dprint's plugins, cargo's deps - one run, all three on a day's
# cooldown: mise.toml's minimum_release_age, dprint's flag, cooldown.toml.
# --bump: the pins are exact, so nothing moves without it. --local: never the
# global config's tools. A major is held either way; widening one is by hand.
# fmt before check: a plugin bump reformats, and the diff is the deliverable.
upgrade: ## bump mise's pins, dprint's plugins and cargo's deps, then check
	@test -z "$$(git status --porcelain)" || { echo 'the tree is dirty; commit first' >&2; exit 1; }
	mise upgrade --bump --local
	mise exec -- dprint config update --yes --reload=plugins --minimum-dependency-age P1D
	cargo cooldown update
	make --no-print-directory fmt
	@git --no-pager diff --stat
	make --no-print-directory check test

clean: ## cargo clean
	cargo clean
