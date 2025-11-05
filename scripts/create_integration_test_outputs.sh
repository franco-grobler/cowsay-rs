#!/usr/bin/env nix-shell
#!nix-shell -i bash -p cowsay

test_dir="$(dirname "$0")/../crates/cowsay/tests/expected_outputs"

phrase="Hello, cowsay-rs!"

declare -a options=(
	"-b,option_borg"
	"-d,option_dead"
	"-g,option_greedy"
	"-s,option_sleepy"
	"-t,option_tired"
	"-w,option_wired"
	"-y,option_young"
)

for cowfile in cows/*; do
	cowname=$(basename "$cowfile" .cow)
	options+=("-f '$cowname',file_${cowname}")
done

rm -r "$test_dir"
mkdir -p "$test_dir"

for option in "${options[@]}"; do
	IFS=',' read -r -a elements <<<"${option}"

	opt=${elements[0]}
	name=${elements[1]}

	echo "Generating test output for ${name} mode with params '${opt}'..."

	eval "cowsay $opt '$phrase' >'${test_dir}/${name}.txt'"
done
