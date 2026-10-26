# Code Review: `cowsay-interpreter` Crate

**Scope:** `crates/cowsay-interpreter` only — the new Perl5-subset interpreter
for cowfile prologues. Integration with `cowsay-template` and the cowfiles
themselves are out of scope for this review.

**Goal:** The interpreter must be correct, idiomatic Rust, and measurably faster
than the existing Perl-based cowfile execution path.

**Overall assessment:** The crate is a promising start but is not
production-ready. Critical correctness bugs in the scanner/parser boundary and
fragile string/here-doc handling will cause real cowfiles to fail or panic. The
public API, error handling, and test/benchmark coverage need substantial work
before this can replace a Perl interpreter.

---

## 1. Correctness

### 1.1 Scanner emits tokens the parser does not recognize (and vice versa)

- **Severity:** Critical
- **Milestone:** Must fix before any cowfile can run end-to-end.

The scanner maps `true`/`false` to `Type::LiteralBoolean`, but
`parser/expressions.rs` looks for `Type::KeywordTrue` / `Type::KeywordFalse`.
`undef` is not recognized as a keyword and becomes a generic `Identifier`, while
the parser expects `Type::KeywordNil`. Additionally, `>`, `*`, and `/` are not
emitted by the scanner at all, even though the parser/evaluator already
understand `MoreThan`, `Multiply`, and `SlashForward`.

Expressions such as `1 > 2`, `3 * 4`, `8 / 2`, `true`, `false`, and `undef`
cannot be parsed. Many real cowfiles will fail during parsing.

**Recommendation:** Audit the `Type` enum and ensure every variant is produced
by exactly one scanner path and consumed by exactly one parser path. Add a
round-trip unit test that scans and parses every operator, keyword, and literal
type.

---

### 1.2 Here-doc scanning is incorrect for common cases

- **Severity:** Critical
- **Milestone:** Must fix before merging any cowfile that uses here-docs.

`lexer/matcher.rs:scan_here_doc` has multiple independent defects:

- `delim_start_idx = start_idx + 2` is wrong for `<<-` (should be `+3`), causing
  the delimiter to include the leading `-`.
- `delimiter_len = self.current_idx - 3 - start_idx` is wrong for `<<-`,
  corrupting the closing delimiter span.
- The closing search uses `find("\n{delimiter}")`, so any line that merely
  _contains_ the delimiter word will prematurely close the here-doc.
- Empty here-docs fail because `"\n{delimiter}"` cannot match when content
  starts immediately with the delimiter.
- `<<'EOC'` does not strip single quotes, while `<<"EOC"` strips double quotes.

**Recommendation:** Rewrite here-doc scanning to:

1. Detect `<<` vs `<<-` up front.
2. Read the delimiter, trimming both single and double quotes consistently.
3. Scan line-by-line, comparing the whole line to the delimiter (allowing
   leading whitespace for `<<-`).
4. Add focused unit tests for each variant, including empty content and
   delimiter-in-content cases.

---

### 1.3 `chop` panics on empty strings

- **Severity:** Critical
- **Milestone:** Must fix before any cowfile uses `chop`.
- **Status**: Done

`stdlib/chop.rs:5` calls `val.split_off(val.len() - 1)`, which underflows when
`val` is empty. Any cowfile that calls `chop` on an empty value will crash.

**Recommendation:** Return an empty string for empty input (matching Perl
behavior) and add a unit test for the empty case.

---

### 1.4 Interpolated-string evaluation processes escapes on substituted values

- **Severity:** Critical
- **Milestone:** Must fix before string interpolation is trustworthy.

`evaluator/eval.rs` evaluates each interpolated part to a string, concatenates
them, and then processes `\n`, `\t`, etc. on the combined result. A variable
whose value contains a literal backslash followed by `n` will have it converted
to a newline. Escapes should only be interpreted in string literals, not in
substituted variable values. The unterminated-escape error also uses a
hard-coded `Span::new(0, 0)`.

**Recommendation:** Apply escape processing only to literal string segments
during scanning, not during evaluation. Store each segment as already-unescaped
text and concatenate without re-processing. Preserve accurate source spans for
escape errors.

---

### 1.5 No reassignment support

- **Severity:** Medium
- **Milestone:** Required for stateful Perl prologues.

The parser only handles `=` during a variable declaration. `$a = 1; $a = 2;`
fails on the second line.

**Recommendation:** Add an assignment expression/statement rule separate from
declaration, and test with a simple counter-style prologue.

---

### 1.6 Missing operators required by real cowfiles

- **Severity:** Medium
- **Milestone:** Align with the cowfiles that are intended to run.

The cowfiles being added use `%`, `?:`, `.=`, `x`, `unless`, `ne`, and function
calls. Most of these are not implemented in the interpreter.

**Recommendation:** Either implement the operators/features used by the target
cowfiles, or stage those cowfiles separately and merge them only when the
interpreter supports them. Maintain a tracked feature matrix.

---

### 1.7 Single- and double-quoted strings do not handle escaped quotes

- **Severity:** Medium
- **Milestone:** Should be fixed before broad cowfile testing.

`scan_string` stops at the first `'` and does not handle `\'`.
`scan_interpolation_string` stops at the first `"` and does not handle `\"`.

**Recommendation:** Add escape-aware scanning and regression tests for
apostrophes and escaped quotes.

---

### 1.8 Trailing comments without a newline produce an error token

- **Severity:** Low
- **Milestone:** Polish before release.

`scanner.rs:advance_line` emits `Type::Error` when it reaches EOF before finding
`\n`. A file ending in `# comment` scans as an error.

**Recommendation:** Treat EOF as a valid comment terminator.

---

### 1.9 Variable names cannot contain digits

- **Severity:** Low
- **Milestone:** Polish before release.

`lexer/utils.rs:is_valid_identifier` only accepts ASCII letters and `_`, so
`$var1` tokenizes as `$var` followed by `1`.

**Recommendation:** Extend identifier validation to allow digits after the first
character.

---

## 2. Performance

### 2.1 Repeated string allocation in interpolation and evaluation

- **Severity:** Medium
- **Milestone:** Address once correctness tests pass; critical for outperforming
  Perl.

`eval.rs` builds interpolated strings by repeatedly concatenating `String`
values. For large cowfiles or many substitutions this causes repeated
reallocation and copying.

**Recommendation:** Collect segments into a `Vec<String>` or `Vec<&str>` and use
`join("")` once, or build with a `String` pre-allocated to an estimated size. If
segments are already owned, `Vec<String>::concat()` is a simple improvement.

### 2.2 No allocation or throughput measurements yet

- **Severity:** Medium
- **Milestone:** Add before any performance claims are made.

There are no benchmarks comparing this interpreter to the Perl path. Without
measurements, claims about being "much faster than Perl" cannot be verified or
defended.

**Recommendation:** See section 6 for a `criterion.rs` benchmark suite that
compares scan, parse, and render throughput against an equivalent Perl
invocation.

---

## 3. Rust Best Practices

### 3.1 `unwrap()` on user-controlled data

- **Severity:** Critical
- **Milestone:** Must fix before the crate is used as a library.

`main.rs` calls `.unwrap()` on `environment.get("the_cow", None)`. If a script
does not define `$the_cow`, the binary panics.

**Recommendation:** Return `Result` from the public API and from `main`, and
propagate the missing-output error with a descriptive message.

---

### 3.2 Errors are swallowed in `main.rs`

- **Severity:** Critical
- **Milestone:** Must fix alongside section 3.1.

`main.rs` uses `let _ = evaluator.execute(statement);` and only prints parser
errors as debug output after partial state has already been printed. Malformed
scripts produce no useful error and may crash.

**Recommendation:** Use `?` to propagate errors. A single clear error message is
better than partial output followed by debug dumps.

---

### 3.3 Dead token variants will fail `deny(warnings)`

- **Severity:** Medium
- **Milestone:** Fix before CI with `warnings = "deny"`.

`Type` contains variants such as `StatementPrint`, `LoopWhile`, `LoopFor`,
`Function`, `Return`, `SlashBackward`, `KeywordUnless`, `KeywordTrue`,
`KeywordFalse`, `KeywordNil`, and `LiteralBoolean` that are never produced or
never consumed.

**Recommendation:** Remove unused variants, or if they are planned, add
`#[allow(dead_code)]` with a `TODO` and target milestone.

---

### 3.4 Library API is undefined

- **Severity:** Critical
- **Milestone:** Must fix before integration with the rest of the workspace.

`lib.rs` exposes a placeholder `add` function and no clean entry point for
executing a cowfile with injected variables.

**Recommendation:** Define a small, stable public API such as:

```rust
pub struct RenderInput<'a> {
    pub source: &'a str,
    pub eyes: &'a str,
    pub tongue: &'a str,
    pub thoughts: &'a str,
}

pub fn render(input: &RenderInput) -> Result<String, InterpreterError> { ... }
```

This allows callers to inject `$eyes`, `$tongue`, `$thoughts`, and `$t` and
retrieve `$the_cow` without knowing the internal environment.

---

### 3.5 Binary scope is unfocused

- **Severity:** Medium
- **Milestone:** Decide before merging.

`src/main.rs` implements a generic REPL that prints to stdout and panics on
missing output. A REPL is probably not needed for a cowfile interpreter embedded
in a cowsay tool.

**Recommendation:** Either remove the REPL and make the binary a thin wrapper
around the library API, or clearly document why a REPL is required.

---

### 3.6 Usage string names the wrong binary

- **Severity:** Low
- **Milestone:** Polish before release.

`main.rs` prints `Usage: rlox [script]`. The crate is `cowsay-interpreter`.

**Recommendation:** Derive the usage string from `env!("CARGO_PKG_NAME")` or
update it manually.

---

## 4. Testing Improvements

### 4.1 Add end-to-end cowfile corpus tests

- **Severity:** Critical
- **Milestone:** Add as soon as the scanner/parser agreement is fixed.

The existing tests cover the scanner and parser in isolation, but there are no
tests that run real cowfiles and compare output.

**Recommendation:** Add a corpus test that loads every `.cow` file, injects
known variables, and asserts that the rendered `$the_cow` matches a snapshot or
at least does not panic/error. This would have caught most of the correctness
issues above.

### 4.2 Round-trip tests for every token type

- **Severity:** Medium
- **Milestone:** Add alongside scanner fixes.

Add a test that, for every `Type` variant, verifies it can be produced by the
scanner and consumed by the parser.

### 4.3 Property-based tests for expression evaluation

- **Severity:** Medium
- **Milestone:** Add after core evaluator is stable.

Use `proptest` or `quickcheck` to generate random arithmetic/boolean expressions
and compare the interpreter's result to a reference evaluator.

### 4.4 Edge-case tests for string and here-doc scanning

- **Severity:** Critical
- **Milestone:** Add with the here-doc rewrite.

Cover empty strings, escaped quotes, empty here-docs, `<<-` with indented
closing delimiters, delimiter words inside content, and quoted delimiters.

### 4.5 Negative tests

- **Severity:** Medium
- **Milestone:** Add once error types are stable.

Assert that specific malformed inputs produce specific, helpful errors: missing
`$the_cow`, invalid syntax, division by zero, etc.

---

## 5. Prioritized Action Plan

| Priority | Item                                                                            | Severity | Milestone        |
| -------- | ------------------------------------------------------------------------------- | -------- | ---------------- |
| 1        | Fix scanner/parser token agreement (`>`, `*`, `/`, booleans, `undef`)           | Critical | Pre-merge        |
| 2        | Return `Result` from the public API and `main`; remove `unwrap()` on `$the_cow` | Critical | Pre-merge        |
| 3        | Rewrite here-doc scanning                                                       | Critical | Pre-merge        |
| 4        | Fix interpolated-string escape handling                                         | Critical | Pre-merge        |
| 5        | Define a stable public library API                                              | Critical | Pre-merge        |
| 6        | Fix `chop` panic on empty strings                                               | Critical | Pre-merge        |
| 7        | Add end-to-end cowfile corpus tests                                             | Critical | Pre-merge        |
| 8        | Remove or annotate dead token variants                                          | Medium   | Pre-merge        |
| 9        | Add reassignment support                                                        | Medium   | Post-merge v2    |
| 10       | Implement missing operators (`%`, `?:`, `.=`, `x`, `unless`, `ne`, calls)       | Medium   | Post-merge v2    |
| 11       | Reduce string allocations in interpolation                                      | Medium   | Performance pass |
| 12       | Add `criterion` benchmarks vs. Perl                                             | Medium   | Performance pass |
| 13       | Decide fate of the REPL / fix usage string                                      | Low      | Polish           |

---

## 6. Benchmark Appendix: `criterion.rs` Suite

The interpreter's explicit goal is to be **much faster than the Perl
interpreter**. The following benchmark suite should be added under
`crates/cowsay-interpreter/benches/`.

### 6.1 `Cargo.toml` additions

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "interpreter"
harness = false
```

### 6.2 `benches/interpreter.rs`

```rust
use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use cowsay_interpreter::{render, RenderInput};

const SAMPLE_COWFILE: &str = r#"
$the_cow = "...";
"#;

const STRESS_COWFILE: &str = r#"
$eyes = "oo";
$tongue = "  ";
$thoughts = "\\ ";
$the_cow = " $thoughts  $eyes\n  $tongue ";
"#;

fn render_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");
    group.measurement_time(Duration::from_secs(10));

    let inputs = [
        ("tiny", "tiny cowfile content"),
        ("sample", SAMPLE_COWFILE),
        ("stress", STRESS_COWFILE),
    ];

    for (name, source) in &inputs {
        let input = RenderInput {
            source,
            eyes: "oo",
            tongue: "  ",
            thoughts: "\\ ",
        };
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_with_input(BenchmarkId::new("rust", name), &input, |b, input| {
            b.iter(|| render(input).unwrap())
        });
    }

    group.finish();
}

fn parse_bench(c: &mut Criterion) {
    c.bench_function("parse stress cowfile", |b| {
        b.iter(|| cowsay_interpreter::parse(STRESS_COWFILE).unwrap())
    });
}

criterion_group!(benches, render_bench, parse_bench);
criterion_main!(benches);
```

### 6.3 Comparison with Perl

To validate the "much faster than Perl" goal, add a shell-driven benchmark in a
`benches/perl_comparison.rs` or CI script:

```bash
#!/usr/bin/env bash
set -euo pipefail

COWFILE="benches/fixtures/stress.cow"
PERL_LOOPS=10000
RUST_LOOPS=10000

echo "Perl ($(perl -v | head -2 | tail -1))"
time perl -e '
    for (1..'$PERL_LOOPS') {
        do "'$COWFILE'";
    }
'

echo "cowsay-interpreter"
time cargo run --release --bin cowsay-interpreter -- "$COWFILE" $RUST_LOOPS
```

Or use `criterion`'s `BenchmarkGroup` to shell out to Perl per iteration and
compare wall-clock time directly. Keep fixtures version-controlled and run both
on the same machine.

### 6.4 Performance targets

Until the interpreter is functionally complete, do not optimize prematurely.
Once correctness tests pass:

- Target at least **2× faster** than Perl for simple cowfiles.
- Target at least **5× faster** for cowfiles with heavy interpolation or many
  here-docs.
- Track allocator pressure using `cargo bench` plus `dhat` or `cargo-flamegraph`
  if throughput targets are missed.

---

## 7. Conclusion

Do not merge this crate for production use until the critical correctness and
API issues are resolved and an end-to-end cowfile corpus test is passing. After
that, the main work is completing the Perl subset needed by the target cowfiles
and proving the performance advantage over Perl through `criterion` benchmarks.
