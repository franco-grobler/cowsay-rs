#!/usr/bin/env bash
set -e

root="$(dirname "$0")/.."

hash_pattern='[a-zA-F0-9]\{40\}'

files=$(rg --type=yaml --files --e="actions/checkout" "${root}/.github")

# name, hash, version
declare -a matrix=(
	"actions\/attest-build-provenance,c074443f1aee8d4aeeae555aebba3282517141b2,v2.2.3"
	"actions\/cache,0057852bfaa89a56745cba8c7296529d2fc39830,v4.3.0"
	"actions\/checkout,8e8c483db84b4bee98b60c0593521ed34d9990e8,v6.0.1"
	"actions\/download-artifact,018cc2cf5baa6db3ef3c5f8a56943fffe632ef53,v6.0.0"
	"actions\/upload-artifact,330a01c490aca151604b8cf639adc76d48f6c5d4,v5.0.0"
	"actions\/setup-node,2028fbc5c25fe9cf00d9f06a71cc4710d4507903,v6.0.0"
	"astral-sh\/setup-uv,85856786d1ce8acfbcc2f13a5f3fbd6b938f9f41,v7.1.2"
	"cargo-bins\/cargo-binstall,b3f755e95653da9a2d25b99154edfdbd5b356d0a,v1.15.10"
	"jetli\/wasm-pack-action,0d096b08b4e5a7de8c28de67e11e945404e9eefa,v0.4.0"
	"oven-sh\/setup-bun,735343b667d3e6f658f44d0eca948eb6282f2b76,v2.0.2"
	"rustsec\/audit-check,69366f33c96575abad1ee0dba8212993eecbe998,v2.0.0"
	"Swatinem\/rust-cache,f13886b937689c021905a6b90929199931d60db1,v2.8.1"
	"taiki-e\/install-action,81ee1d48d9194cdcab880cbdc7d36e87d39874cb,v2.62.45"
)

for option in "${matrix[@]}"; do
	IFS=',' read -r -a elements <<<"${option}"

	name=${elements[0]}
	hash=${elements[1]}
	version=${elements[2]}

	pattern="uses: ${name}@${hash_pattern} .*"
	replace="uses: ${name}@${hash} # ${version}"

	for file in $files; do
		sed -i "s/${pattern}/${replace}/" "$file"
	done
	sed -i \
		"s/\"${name}\" = \"${hash_pattern}\".*/\"${name}\" = \"${hash}\" # ${version}/" \
		"${root}/dist-workspace.toml"

done
