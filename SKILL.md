---
name: google-rust-style
description: >-
  Write, refactor, review, and maintain idiomatic Rust following Google's Rust
  Style Guide, the official Rust style conventions, and Rust API Guidelines.
  Use for Rust .rs files, Cargo crates and workspaces, Rust API design,
  ownership and borrowing, error handling, naming, modules, traits, unsafe
  reviews, and rustfmt/clippy/test checks. Apply Chromium- or Fuchsia-only
  rules only when working in those environments.
metadata:
  version: "1.0.0"
  source: "https://google.github.io/styleguide/rustguide.html"
  source_checked: "2026-10-22"
---

# Google-style Rust engineering

Write Rust that is readable, maintainable, straightforward to refactor at scale,
compatible with idiomatic Rust, and easy to understand without IDE navigation.
Favor clear intent over cleverness. This skill is an *operational adaptation*
of the Google Rust Style Guide, not a verbatim or official Google publication.

## Apply the right rules

1. Determine whether the code is a **reusable library** or an **application**.
   - For libraries, apply API-design preferences strongly because downstream
     callers cannot be assumed to be under your control.
   - For applications, prefer directness over generic APIs, elaborate traits,
     or abstractions that serve no actual callers.
2. Identify environment: ordinary Cargo project, Chromium, Fuchsia, Android,
   or another codebase. Rules tagged `[chrome]` or `[Fuchsia]` apply **only**
   to their stated platform. Do not forbid `async`/`.await` in ordinary Rust,
   and do not impose Chromium's flat source layout on a Cargo project.
3. Inspect existing `Cargo.toml`, `rust-toolchain*`, `rustfmt.toml`,
   `.rustfmt.toml`, `clippy.toml`, `.clippy.toml`, public APIs, tests,
   neighboring modules, and team conventions before editing.
4. Respect user instructions and established repository conventions. Use
   Google's guide and the official Rust style/API guidelines to resolve gaps.
   Never silently force a repository-wide style migration in a local task.
5. Use stable Rust by default. Do not enable unstable features or nightly-only
   formatter settings merely to satisfy a cosmetic preference.

Full rules and exceptions: [references/rules.md](references/rules.md).
Code examples: [references/examples.md](references/examples.md).
Review checklist: [references/review-checklist.md](references/review-checklist.md).

## Default implementation rules

### API and module design

- Design around the actual caller. Prefer concrete, small APIs over premature
  abstractions in application code.
- Name a leaf module/file after its primary item when one clearly dominates.
  Keep parent modules mostly for `mod` declarations and `pub use`; a small
  amount of wiring logic is reasonable.
- Avoid glob imports. Limited exceptions include `use super::*` in inline
  test/submodules and re-exporting from a non-leaf module when justified.
- Group imports from the same module, without nested paths in `{...}`.
- Prefer `module::function()` for free functions from other modules; import
  type names and use them without qualification unless ambiguity warrants it.
- Import a trait locally when its methods are only used in one function.

### Ownership, parameters, and return values

- Prefer unique ownership and borrowing to `Rc`/`Arc` where practical. Shared
  ownership is valid when it materially simplifies the design or is required
  for concurrent use; justify the choice, do not blanket-ban it.
- Use `.clone()` for genuine ownership needs, not to avoid thinking through
  borrow lifetimes. Allow tactical cloning when borrow complexity is worse.
- Accept `&str` instead of `&String`, `&[T]` instead of `&Vec<T>`, `&T`
  instead of `&Box<T>`, and `Option<&T>` instead of `&Option<T>` when appropriate.
- In reusable libraries, consider `impl Iterator` returns and `IntoIterator`
  inputs when they meaningfully increase flexibility. In apps, a `Vec<T>` or
  `&[T]` may be clearer and perfectly acceptable.
- Avoid gratuitous `impl Into<T>` parameters in application code. In public
  library APIs, use it selectively for tangible call-site ergonomics.
- Use a meaningful `enum` instead of a `bool` when a call would be ambiguous.

### Errors, invariants, and safety

- `Option<T>`: expected absence where an error explanation would not help.
- `Result<T, E>`: recoverable failure requiring diagnostics or differentiation.
- `panic!`/`expect`: violated invariant or genuine programmer error; never
  use panics for routine external/input failures. Prefer `expect("reason")`
  over an unexplained `unwrap()` when an invariant really must panic.
- Prefer `if let`, `match`, or `let ... else` to check-then-`unwrap()`.
- Avoid `unsafe` if an equivalent safe design exists. If necessary, isolate
  it behind a safe abstraction, document invariants, and add a `// SAFETY:`
  justification adjacent to each unsafe block. Require evidence for
  performance-motivated unsafe.
- Do not rely on `catch_unwind` for ordinary recoverable error handling.
  Google's first-party guidance prohibits it except narrow defense-in-depth.

### Types, traits, and public compatibility

- Prefer descriptive named fields over opaque tuples. Use a newtype to encode
  semantically distinct IDs, units, or values that callers could confuse.
- Implement standard library traits (`FromStr`, `From`, `Default`, etc.) when
  semantics match; use `#[derive(...)]` instead of hand-written boilerplate.
- Define traits for a genuine polymorphism, abstraction boundary, or extension
  use case. Do not introduce traits just to categorize methods.
- In library APIs, favor generics over `dyn Trait` unless build size,
  heterogeneity, or another concrete trade-off argues otherwise.
- For public cross-team APIs that are likely to evolve, consider
  `#[non_exhaustive]` on enums and public-field structs; honor its documented
  exceptions and construction ergonomics.
- Use `#[must_use]` when discarding a result would make the call incorrect;
  avoid redundant annotation when the returned type already has it.

### Clarity, formatting, and documentation

- Run `cargo fmt`; use rustfmt instead of manual alignment, generally using
  100-column style when consistent with the repository.
- Follow Rust naming (`snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`) and
  expressive method conventions (`as_`, `to_`, `into_`, `try_new`, etc.).
- Document nontrivial public APIs using `///`, modules using `//!`, and unsafe
  contracts and possible panics explicitly. Do not add comments that merely
  restate the code.
- Prefer a short iterator chain when it expresses intent; use an ordinary
  `for` loop if a complex chain obscures mutation or failure handling.
- Prefer enough type annotations to make `parse()`, `into()`, and complex
  iterator results understandable.
- Use standard `format!` / `write!` for multi-part concatenation and literal
  substitutions when appropriate.
- Avoid proc macros and bespoke code generation unless ordinary Rust would
  be substantially worse to maintain.

## Execution workflow

**Before modifying code**

1. Read the relevant modules and call sites. Identify exported vs private API,
   ownership model, error boundaries, concurrency, and caller constraints.
2. Explain material design decisions only when the task warrants it; otherwise
   implement directly and keep changes scoped.
3. Reuse existing dependencies and established abstractions where suitable.
   Avoid redundant crates and unnecessary compatibility layers.

**While implementing**

4. Make the smallest coherent change. Favor idiomatic, explicit Rust;
   minimize allocations/clones without sacrificing legibility.
5. Add focused tests for normal, boundary, and error cases. Use the project's
   testing framework; standard Cargo projects can use `#[test]`/`#[tokio::test]`
   as appropriate. Google-specific harnesses apply only to their projects.
6. If breaking a public API, identify impact and provide a migration path.

**After implementing**

7. Check formatting, linting, and tests where the tools are available:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```

   Or run `scripts/check.sh` from the Skill's directory, passing the Cargo
   project directory. For very large repositories, select the impacted package
   and follow the existing CI/testing procedure rather than running unrelated
   workspace jobs.
8. Verify no new panic paths on untrusted data, unjustified `unsafe`, hidden
   unnecessary clones, leaked implementation details, or unused dependencies.
9. Report: changed behavior, key design choices, verification commands/results,
   and any intentional guideline exceptions. Never claim tests passed unless
   they actually ran successfully.

## When reviewing rather than writing

- Report concrete findings in severity order, with file/line locations,
  effect on correctness or maintainability, and an actionable fix.
- Distinguish **Google guidance**, **platform-specific constraints**,
  **repository conventions**, and **your own engineering recommendation**.
- Don't label a normal Cargo project noncompliant for using `async`, standard
  `src/foo/mod.rs` layouts, or `#[test]`.
- Do not request broad refactors for subjective style alone.

## Source precedence and interpretation

1. Safety/correctness, project requirements, and explicit user constraints.
2. Established project/toolchain conventions and available CI configuration.
3. Google's general Rust Style Guide (linked in frontmatter).
4. Official Rust Style Guide and Rust API Guidelines where nonconflicting.
5. Optional skill-added engineering suggestions, clearly distinguished from
   statements directly attributable to Google.

If an apparent rule conflicts with the context or would harm readability,
check its scope and exceptions in `references/rules.md` before applying it.
