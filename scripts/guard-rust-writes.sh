#!/usr/bin/env bash
# Agent pre-command hook: block shell commands that would modify Rust source.
# The human writes the Rust in this repo; agents only review (see AGENTS.md).
#
# Reads hook JSON on stdin; accepts `.tool_input.command` (Claude Code, Gemini CLI)
# or `.command` (Cursor). Exits 2 with a reason on stderr to block.
cmd=$(jq -r '.tool_input.command // .command // empty')

block() {
  echo "Blocked: $1. Rust files in this repo are written by the human only — give review/guidance instead." >&2
  exit 2
}

# Tools that rewrite .rs files in place
if printf '%s' "$cmd" | grep -Eq '(cargo[[:space:]]+(\+[^[:space:]]+[[:space:]]+)?(fmt|fix)\b|clippy[^|;&]*--fix|rustfmt\b)' &&
  ! printf '%s' "$cmd" | grep -Eq -- '--check'; then
  block "command would rewrite Rust source (fmt/fix)"
fi

# Generic write patterns aimed at a .rs file
if printf '%s' "$cmd" | grep -Eq '[[:alnum:]_*]\.rs\b' &&
  printf '%s' "$cmd" | grep -Eq '(>|\btee\b|\bsed[^|;&]*-i|\bperl[^|;&]*-i|\b(cp|mv|rm|touch|install|patch|truncate)\b|git[[:space:]]+(apply|checkout|restore|stash|reset))'; then
  block "command appears to write to a .rs file"
fi

exit 0
