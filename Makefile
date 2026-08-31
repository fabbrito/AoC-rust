YEAR ?= 2023
DAY ?=

SHFMT_FLAGS = -i 0 -ci
SHELLCHECK_FLAGS = -x -P scripts
SCRIPTS = $(wildcard scripts/*.sh) $(wildcard .githooks/*)

# printf reads 08 and 09 as bad octal, so strip a leading zero before padding.
pad = $(shell printf '%02d' $(patsubst 0%,%,$(DAY)))

# scope: nothing = whole workspace, YEAR = one year, YEAR + DAY = one day.
# YEAR only narrows when passed explicitly - the ?= default above must not count.
year_set = $(filter command line environment,$(origin YEAR))
scope = $(if $(DAY),-p aoc$(YEAR) --bin day$(pad),$(if $(year_set),-p aoc$(YEAR),--workspace))
# --all-targets and --tests both override --bin rather than narrowing it. For one day,
# --profile test is what compiles the bin together with its own #[cfg(test)] module.
lint_scope = $(scope) $(if $(DAY),--profile test,--all-targets)
fmt_scope = $(if $(year_set)$(DAY),-p aoc$(YEAR),--all)

# Not adopted (yet): nursery is unstable, and the other three each cost more
# ceremony than they buy in a puzzle. Advisory - see the `pedantic` target.
ADVISORY = -W clippy::nursery \
           -W clippy::arithmetic_side_effects \
           -W clippy::as_conversions \
           -W clippy::cast_precision_loss \
           -W clippy::cast_possible_truncation \
           -W clippy::cast_sign_loss

.PHONY: help hooks day run test check lint pedantic fmt clean

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

##@ Setup
help: ## show this help
	@awk "$$HELP_AWK" $(lastword $(MAKEFILE_LIST))

hooks: ## enable .githooks for this clone
	git config core.hooksPath .githooks
	@echo 'hooks enabled - skip one commit with --no-verify'

##@ Solve
day: ## scaffold a day - needs YEAR and DAY
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	scripts/new-day.sh $(YEAR) $(DAY)

run: ## run one day in release - DAY required
	@test -n '$(DAY)' || { echo 'set DAY=<n>' >&2; exit 1; }
	cargo run --release $(scope)

test: ## cargo test
	cargo test $(scope)

##@ Quality
check: ## cargo check - types only, no lints
	cargo check $(lint_scope)

# Clippy levels live in [workspace.lints.clippy]; -D warnings is what catches the
# rest - rustc's own dead_code, unused_variables and friends.
lint: ## clippy + rustfmt/shfmt/shellcheck/dprint - read only
	cargo clippy $(lint_scope) -- -D warnings
	cargo fmt $(fmt_scope) --check
	shfmt $(SHFMT_FLAGS) -d $(SCRIPTS)
	shellcheck $(SHELLCHECK_FLAGS) $(SCRIPTS)
	dprint check

# Advisory only - it flags plenty that is fine in a puzzle. Mine it for
# candidates after a day lands; promote a rule by moving it into Cargo.toml.
pedantic: ## the lints make lint does not deny - advisory
	cargo clippy $(lint_scope) -- $(ADVISORY)

fmt: ## format rust + shell + markdown - writes
	cargo fmt --all
	shfmt $(SHFMT_FLAGS) -w $(SCRIPTS)
	dprint fmt

clean: ## cargo clean
	cargo clean
