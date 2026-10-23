# Rust code review checklist

Review the requested diff first. Avoid turning a focused review into a
repository-wide stylistic rewrite.

## Correctness and safety (highest priority)

- [ ] No `unwrap`/`expect`/panic on routine external or invalid user input.
- [ ] `Option`/`Result` semantics preserve actionable failure information.
- [ ] Borrowed data remains valid; no needless workarounds that obscure logic.
- [ ] `unsafe` (if any) has clear necessity, isolation, invariants, and `SAFETY:`.
- [ ] No dependence on `catch_unwind` to recover ordinary errors.
- [ ] Concurrent state is intentionally shared, synchronized, and bounded.
- [ ] Edge-case and error-path tests cover changed behavior.

## Public API quality

- [ ] Code classified as application or library, with appropriate rigor.
- [ ] Borrowed parameters use `&str`/`&[T]`/`&T` when appropriate.
- [ ] Enums replace ambiguous boolean toggles where meaning matters.
- [ ] Distinct domain identifiers use newtypes when confusion is plausible.
- [ ] Standard traits and `derive` replace redundant custom implementations.
- [ ] Traits, dynamic dispatch, macros, and generics have actual use cases.
- [ ] Public cross-team API evolution considered (`#[non_exhaustive]`, etc.).
- [ ] `len()` accompanies `is_empty()` when applicable.
- [ ] Public docs explain unusual preconditions, panics, and safety contracts.

## Maintenance and readability

- [ ] Non-leaf modules primarily organize declarations/re-exports.
- [ ] Imports are explicit, grouped, and easy to trace.
- [ ] Meaningful names; no unexplained abbreviations or clever one-liners.
- [ ] Iterator chains remain readable; imperative loops used if clearer.
- [ ] Deep clones, allocations, and `Rc`/`Arc` are justified where significant.
- [ ] Scope is focused; no unnecessary dependencies or speculative rewrites.
- [ ] Platform restrictions applied only to their intended environments.

## Verification

- [ ] Formatter checked with repository-appropriate tools.
- [ ] Clippy/linter run against the relevant targets when available.
- [ ] Tests executed for the affected scope, with results accurately reported.
- [ ] Build/toolchain/CI constraints noted rather than silently bypassed.

## Suggested review output

For each meaningful finding:

`[P0/P1/P2/P3] <concise issue> — path/to/file.rs:line`

`Impact: <what can go wrong or become difficult>`

`Fix: <smallest practical, concrete change>`

`Basis: <Google general | Chrome-only | Fuchsia-only | repo convention | advice>`

P0 = critical correctness/security blocker; P1 = major issue; P2 = important
maintainability/API concern; P3 = optional polish. Do not call a preference a
P0/P1 problem without explaining a concrete consequence.
