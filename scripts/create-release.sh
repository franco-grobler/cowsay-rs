#!/usr/bin/env bash
set -e

root="$(dirname "$0")/.."
changelog_file='CHANGELOG.md'

version=$(git-cliff --bumped-version --config="${root}/cliff.toml")
commit_msg="chore(release): prepare release version ${version}"

sed -Ei "s/^version = .*$/version = \"${version#v}\"/" "${root}/Cargo.toml"

git-cliff --bump \
	--config "${root}/cliff.toml" \
	--with-commit "${commit_msg}" \
	--output="${changelog_file}"

git add "${root}/${changelog_file}"
git add "${root}/Cargo.toml"

git commit -m "${commit_msg}"
git tag -s -a "v$version" -m "Release $version" -m "$(cat "${root}/${changelog_file}")"
git tag -v "v${version}"

git push
git push --tags
