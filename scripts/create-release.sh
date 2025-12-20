#!/usr/bin/env bash
set -e

root="$(dirname "$0")/.."

git-cliff --bump --config "${root}/cliff.toml"
git add "${root}/CHANGELOG.md"
git commit -m "chore(release): add changelog" -m 'prepare for release'
