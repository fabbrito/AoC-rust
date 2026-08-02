#!/usr/bin/env bash
#
# Scaffold a day: <year>/src/bin/dayNN.rs and <year>/inputs/dayNN.txt
#
# usage: scripts/new-day.sh <year> <day>

root=${BASH_SOURCE[0]%/*}/..

year=$1
day=$2

if [[ -z $year || -z $day ]]; then
	echo "usage: ${0##*/} <year> <day>" >&2
	exit 1
fi

if [[ ! $day =~ ^[0-9]+$ ]] || ((10#$day < 1 || 10#$day > 25)); then
	echo 'day must be 1-25' >&2
	exit 1
fi

n=$((10#$day))
printf -v padded '%02d' "$n"

if [[ ! -d $root/$year ]]; then
	echo "no crate for $year - add it to the workspace first" >&2
	exit 1
fi

src="$root/$year/src/bin/day$padded.rs"
input="$root/$year/inputs/day$padded.txt"

if [[ -e $src ]]; then
	echo "exists: $src"
else
	template=$(<"$root/templates/day.rs") || exit 1
	echo "${template//DAY/$n}" >"$src" || exit 1
	echo "created: $src"
fi

if [[ ! -e $input ]]; then
	: >"$input" || exit 1
	echo "created: $input"
fi

echo "run: cargo run -p aoc$year --bin day$padded"
