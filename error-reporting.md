# Error Reporting Plan for `crates/mooscript`

Beautiful, rustc-style diagnostics for the `mooscript` (a.k.a.
`cowsay-interpreter`) crate, matching the look used by **ruff**, **ty**, and the
rustc ecosystem.

## Decisions

- **Library**: `annotate-snippets` (the rustc renderer) — produces the
  `--> file:line:col`, `│`, `^` look used by ruff/ty/rustc. Pure-rendering
  crate; no trait bloat; no proc-macro derives; fits the existing hand-rolled
  `Span` cleanly.
- **Scope**: Unify lexer + parser + runtime errors under one diagnostics stream;
  report them together at the end.
- **Type shape**: Keep `RuntimeError` as the internal typed enum (for `Result`
  flow and `match` arms); add a separate `Diagnostic` struct for rendering;
  convert at the boundary via `From<RuntimeError> for Diagnostic`.

## Current state (baseline)

- `Span` (`ast/token.rs:5`) is byte offsets `{ start, end }` — **no line/column
  mapping anywhere**.
- `RuntimeError` (`result.rs:35`) mixes parse + runtime errors; every variant
  carries a `Span`, but `Display` (`result.rs:88`) **discards the span**.
- Collection is a `Vec<RuntimeError>` on `Parser` (`parser/core.rs:13`).
  `add_error` (`core.rs:112`) eagerly `eprintln!`s on collection (no span
  printed) and pushes a clone.
- Runtime errors are **not** collected — they propagate via `Result`, and
  `main.rs:114` drops them with `let _ =`.
- Rendering: none. `main.rs:126` does
  `eprintln!("errors: {:?}", parser.errors())` — a `Debug` dump with no source
  context, no caret.
- No deps at all in `mooscript/Cargo.toml`; strict lints (`missing_docs`,
  `missing_debug_implementations`, `pedantic`, `nursery`, `cargo`).
- Lexer errors are emitted as `Type::Error` tokens, not `RuntimeError`s — they
  only surface indirectly as "unexpected token" parse errors.

## Architecture

New `src/diagnostics/` module:

```
src/diagnostics/
├── mod.rs        // re-exports, From<RuntimeError>
├── source.rs     // SourceFile: owns String, line_starts, byte->(line,col)
├── diagnostic.rs // Diagnostic { severity, code, message, span, labels[], notes[] }, Severity, Label
└── reporter.rs   // Reporter: accumulates + renders via annotate-snippets
```

### `SourceFile` (`diagnostics/source.rs`)

Owns the source `String` (so it lives long enough for rendering), precomputes
`line_starts: Vec<usize>` on construction. Exposes:

- `line_column(byte_offset) -> (usize, usize)` — 1-based line, 1-based column by
  **char** (UTF-8 aware).
- `line_range(line) -> Range<usize>` and `slice(range) -> &str`.
- Binary-search over `line_starts` for line lookup.

### `Diagnostic` (`diagnostics/diagnostic.rs`)

```rust
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    pub span: Span,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

pub enum Severity { Error, Warning, Advice }

pub struct Label { span: Span, text: String }
```

Derive `Debug, Clone`. Document every public item (strict `missing_docs` lint).

`impl From<RuntimeError> for Diagnostic` maps each variant:

- `ExecutionError { message, span }` → `Diagnostic::error(span, message)`
- `ParsingError { message, span }` → `Diagnostic::error(span, message)`
- `TypeMismatch { expected, found, span }` → message + secondary label
- `DivisionByZero { span }` → fixed message at span
- `UndefinedFunction { name, span }` → message naming the function
- `UndefinedVariable { name, span: Option<Span> }` → message naming the
  variable; render with **no underline** when `span` is `None`
- `UndefinedControlSequence { span }` → fixed message at span

### `Reporter` (`diagnostics/reporter.rs`)

```rust
pub struct Reporter {
    source: SourceFile,
    diagnostics: Vec<Diagnostic>,
}
```

Methods:

- `new(source: SourceFile) -> Self`
- `push(&mut self, d: Diagnostic)`
- `extend(&mut self, errs: impl IntoIterator<Item = RuntimeError>)` — converts
  via `From`.
- `has_errors(&self) -> bool`
- `exit_code(&self) -> i32` — `1` if any `Severity::Error`, else `0`.
- `emit(&self, writer: &mut impl Write)` — renders each diagnostic via
  `annotate_snippets`.

Rendering: for each diagnostic, map its `Span` to the line range, slice the
source, build `annotate_snippets::{Snippet, Annotation}` with **local** byte
offsets (annotate-snippets uses byte ranges within the provided source slice),
attach labels and notes as footer, render with `Renderer::styled()` (color,
terminal) or `Renderer::plain()` (tests/snapshots).

Graceful degradation: when `span` is `None` (UndefinedVariable) or
`Span::new(0, 0)` (placeholder), render the message with **no source
annotation** rather than crashing or pointing at byte 0.

## Wiring changes

1. **Lexer** (`lexer/scanner.rs`): add `errors: Vec<RuntimeError>` field. Where
   `Type::Error` tokens are produced, also push a real `RuntimeError` carrying
   the span (keep the `Error` token so parser synchronization still works). Add
   `Scanner::errors() -> &[RuntimeError]`.
2. **Parser** (`parser/core.rs:117`): **delete the
   `eprintln!("Error encountered: {message}")`** — errors are no longer printed
   eagerly; rendered once at the end. Keep `Vec<RuntimeError>` on the parser;
   convert at the reporter boundary.
3. **Evaluator** (`evaluator/eval.rs:131-135`): the `Span::new(0,0)` placeholder
   for escape errors — make the reporter render degraded (no underline) when
   span is `(0, 0)`. Plumbing the real span through escape handling is a flagged
   follow-up (invasive).
4. **`main.rs:run()`**: rewrite to:
   - `SourceFile::new(name, source.to_owned())` + `Reporter::new(source_file)`
   - `reporter.extend(scanner.errors())` after scan
   - `reporter.extend(parser.errors())` after parse
   - per-statement eval:
     `if let Err(e) = evaluator.execute(stmt) { reporter.push(e.into()); }` —
     **continue on error** (collects one error per top-level statement; may
     cascade, acceptable tradeoff)
   - skip eval entirely if there are lex/parse errors
   - at end: if `reporter.has_errors()` → `reporter.emit(&mut io::stderr())` +
     `exit(1)`; else print `cow_frame`
   - remove `eprintln!("Environment: …")` / `eprintln!("tokens: …")` debug dumps
     (gate behind a `--debug` flag if you want to keep them)
5. **`the_cow` `unwrap()` at `main.rs:120-121`**: out of scope for this plan,
   but flag — should become a graceful diagnostic. Left untouched.

## Dependencies

- Add `annotate-snippets = "0.11"` to `workspace.dependencies` in root
  `Cargo.toml`; reference it from `crates/mooscript/Cargo.toml`.
- No `thiserror`/`anyhow` — keep the hand-rolled `impl Error for RuntimeError`.
- Note: workspace `cowsay-interpreter = { path = "crates/cowsay-interpreter" }`
  points at a non-existent dir (the package lives in `crates/mooscript`) —
  separate bug, not touched here.

## Ordered steps

1. Add dep to workspace + mooscript `Cargo.toml`.
2. `diagnostics/source.rs` — `SourceFile` + line/col mapping (UTF-8 aware).
3. `diagnostics/diagnostic.rs` — `Diagnostic`, `Severity`, `Label`.
4. `From<RuntimeError> for Diagnostic` (in `diagnostics/mod.rs` or `result.rs`).
5. `diagnostics/reporter.rs` — `Reporter` + annotate-snippets rendering,
   graceful handling of `None`/`(0, 0)` spans.
6. Wire `diagnostics` into `lib.rs`.
7. Lexer: collect real errors alongside `Type::Error` tokens.
8. Parser: drop eager `eprintln!`.
9. `main.rs`: rewrite `run()` to use `SourceFile` + `Reporter`, per-statement
   eval with collection, emit + exit code.
10. Snapshot tests (`Renderer::plain()`, `pretty_assertions`) for: unexpected
    token, unterminated string, type mismatch, division by zero, undefined
    variable.
11. `cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test && cargo doc`
    — strict lints (`missing_docs`, `missing_debug_implementations`,
    `pedantic`/`nursery`) require doc comments and `Debug` derives on every new
    public item.

## Open items to flag

- **`Span::new(0, 0)` in escape-error path** — rendered degraded now; real fix
  needs span plumbing through `Evaluator` escape handling (follow-up).
- **`the_cow` `unwrap()`** — separate concern, left alone.
- **Per-statement runtime error continuation** may cascade; revisit if noisy.

## Example target output

```
error: expected ';' after expression
  --> input.cow:1:11
   |
 1 | let x = 1
   |           ^
```

```
error: undefined variable `foo`
  --> input.cow:3:9
   |
 3 | print foo
   |        ^^^
```
