#!/usr/bin/env bash
set -ex

root="$(dirname "$0")/.."
changelog_file='CHANGELOG.md'

version=$(git-cliff --bumped-version --config="${root}/cliff.toml")
commit_msg="chore(release): prepare release version ${version}"

sed -Ei "s/^version = .*$/version = \"${version#v}\"/" "${root}/Cargo.toml"

git-cliff --bump \
	--config "${root}/cliff.toml" \
	--with-commit "${commit_msg}" \
	--output="${changelog_file}"

cargo update --workspace

git add "${root}/${changelog_file}"
git add "${root}/Cargo.toml"
git add "${root}/Cargo.lock"

git commit -s -m "${commit_msg}"
git tag -s -a "${version}" -m "Release ${version}" -m "$(cat "${root}/${changelog_file}")"

git push
git push --follow-tags
