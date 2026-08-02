# Advent of Code — Rust

Cargo workspace, one crate per year (`aoc2023`, `aoc2024`, `aoc2025`), one binary per day, plus a
shared `aoc` lib crate.

```
2023/
  inputs/day01.txt      # puzzle input (gitignored)
  src/bin/day01.rs      # part1 / part2 / main / tests
aoc/                    # shared helpers (input loading)
templates/day.rs        # day scaffold
scripts/new-day.sh
Makefile
dprint.json             # markdown formatting
```

## Usage

```sh
make day YEAR=2023 DAY=1               # scaffold src/bin/day01.rs + inputs/day01.txt
make run YEAR=2023 DAY=1               # cargo run --release -p aoc2023 --bin day01
make test YEAR=2023                    # example tests
make check                             # clippy -D warnings + rustfmt check
make fmt / make lint                   # rust + shell + markdown (dprint)
```

Or drive cargo directly: `cargo run -p aoc2023 --bin day01`, `cargo test`.

Paste the puzzle input into `<year>/inputs/dayNN.txt`. `aoc::input!(N)` resolves it relative to the
year crate, so the run directory doesn't matter.
