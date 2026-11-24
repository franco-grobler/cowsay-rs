#!/usr/bin/env bash
set -e

root="$(dirname "$0")/.."

hash_pattern='[a-zA-F0-9]\{40\}'

files=$(rg --type=yaml --files "${root}/.forgejo")

# name, hash, version
declare -a matrix=(
	"actions\/checkout,34e114876b0b11c390a56381ad16ebd13914f8d5,v4.3.1"
	"oven-sh\/setup-bun,735343b667d3e6f658f44d0eca948eb6282f2b76,v2.0.2"
	"Swatinem\/rust-cache,f13886b937689c021905a6b90929199931d60db1,v2.8.1"
	"taiki-e\/install-action,81ee1d48d9194cdcab880cbdc7d36e87d39874cb,v2.62.45"
)

for option in "${matrix[@]}"; do
	IFS=',' read -r -a elements <<<"${option}"

	name=${elements[0]}
	hash=${elements[1]}
	version=${elements[2]}

	pattern="uses: https:\/\/github.com\/${name}@${hash_pattern} .*"
	replace="uses: https:\/\/github.com\/${name}@${hash} # ${version}"

	for file in $files; do
		sed -i "s/${pattern}/${replace}/" "$file"
	done
done
