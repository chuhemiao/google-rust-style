# Select validation by changed behavior

This is skill-added engineering guidance. Choose the smallest existing check
that exercises the changed boundary, then add evidence for uncovered material
risks. Preserve repository commands, toolchain, and supported configurations.

| Changed boundary | Useful evidence | Limit |
| --- | --- | --- |
| Formatting and lint policy | Repository fmt and Clippy checks | Neither proves behavior or soundness |
| Ordinary Rust behavior | Focused unit/integration tests; relevant doctests for public examples | Tests cover exercised inputs and paths |
| Feature-gated or target-specific code | Compile/test the affected supported configurations | A host/default build omits disabled and target-gated code |
| MSRV or `no_std` support | Repository MSRV build; declared no-default-feature/target checks | Newest stable and host `std` builds do not establish these claims |
| Async lifecycle | Controlled cancellation, partial progress, shutdown and error-path tests | A passing schedule is not exhaustive concurrency proof |
| Unsafe memory or FFI | Wrapper regressions and actual foreign integration; Miri when applicable | An interpreter run does not establish foreign behavior or every architecture |

## Select the supported matrix

Read `Cargo.toml`, toolchain files, CI and support documentation. Identify
package, edition, MSRV, targets, default/optional features, and `no_std` promises.
Use the project's actual supported combinations: `--all-features` can enable
mutually exclusive features and is not a universal gate. `--all-targets` selects
Cargo target kinds, not every architecture. `--no-default-features` alone does
not prove the library builds without `std` on its supported embedded target.
A cross-target compile proves less than running tests on that target.

Do not silently install toolchains/targets or change dependencies, lockfiles,
MSRV, features, or runtime to make a check pass. Follow authorized repository
workflows and report unavailable environments. For ordinary baseline checks,
see the commands and optional helper in [../SKILL.md](../SKILL.md).

## Defect and regression evidence

For a reproducible defect, establish the cause from the real owner and callers.
Before repair, run a focused regression that fails for the reported behavior;
a setup error or unrelated compilation failure is not behavioral RED. Check
normal, invalid, boundary and forbidden outcomes where the contract needs them.
Scan the affected owner and direct consumers for the same cause, then keep the
repair scoped to verified occurrences. When reproduction is unavailable or
unsafe, state why, run available checks, and distinguish a proposed correction
from a proven behavioral fix.

After two failures under the same approach, inspect new evidence and change the
hypothesis or report the blocker before retrying. Do not hide failures through
renaming, weaker assertions, removed checks, or unrelated refactors.

## Optional deeper checks

Use tools already supported by the project when their evidence addresses the
changed risk; none is mandatory for every Rust edit:

- [Miri](https://github.com/rust-lang/miri): exercised unsafe memory behavior;
  target, external-operation support and execution coverage remain limited.
- [cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html): input exploration
  for parsers and unsafe wrappers; record seed corpus, duration and target.
- [Rust sanitizers](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html):
  instrumented runtime checks where toolchain/target support permits.
- [Loom](https://docs.rs/loom/latest/loom/): schedule exploration for code modeled
  with its primitives; a model is not automatically proof of production code.

Do not add a new dependency or nightly requirement solely to tick a checklist.

## Report only fresh evidence

Run relevant checks after the final material edit. Report exact command, scope,
result and any skip, failure, retry or flakiness. Distinguish existing failures
from new regressions with evidence. Do not reuse a pre-edit pass as proof of the
final change, equate broad suite success with coverage of a missing mechanism,
or claim a skipped tool passed. Explain remaining unsupported configurations or
unverified behavior in the final result.
