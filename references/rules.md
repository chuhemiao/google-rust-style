# Google Rust Style Guide — working rulebook

This is an actionable *summary*, not a replacement for the source.
Source: https://google.github.io/styleguide/rustguide.html
Checked: 2026-10-22.

Severity language in this reference is advisory. The original guide often
states preferences and exceptions rather than universal compile-time rules.

## A. Scope first

- General Google guidance balances consistency, readability, maintainability,
  tooling, and large-scale automated refactoring.
- For reusable **library code**, public API design guidance should be treated
  as binding in the intended Google context.
- For **application code**, API guidance is strongly recommended, but
  simplicity/directness typically wins over speculative reusability.
- `[chrome]`: Chromium-only requirement; do not generalize to standard Cargo.
- `[Fuchsia]`: Fuchsia-only requirement; do not generalize elsewhere.
- Follow the official Rust Style Guide and Rust API Guidelines unless they
  conflict with the Google guidance.

## B. Code organization and imports

1. **Small crates [chrome].** Chromium favors small `rust_library` targets,
   potentially a single file per abstraction; larger targets are allowed when
   coherence or foundation needs justify them.
2. **Flat file layout [chrome].** Chromium keeps files in one directory per
   `rust_library`; deliberately unlike normal Cargo module layout. Do **not**
   enforce this on standard Cargo crates.
3. Name a leaf module/file after its main declared item if there is one.
4. Keep non-leaf modules mostly declarations and re-exports. Small glue code
   is fine. Inline tests/cross-language bridge submodules do not by themselves
   make a module non-leaf in the intended sense.
5. Avoid glob `use path::*`. Exceptions: inline `super::*`, enum variants in
   relevant method bodies, `pub use submodule::*` in non-leaf modules, a
   narrowly scoped private C++ interop namespace merge, or third-party
   preludes approved by repository owners.
6. Import items from one module per `use` statement/group. Good:
   `use foo::bar::{Bar, Baz};` and `use foo::Foo;`. Avoid
   `use foo::{Foo, bar::Bar};`.
7. Prefer function-local trait imports when only locally useful.
8. Call free functions from another module as `module::function()`, unless
   the name is sufficiently self-explanatory or the documented special
   exceptions apply.
9. Import type names directly (`use regex::Regex;` then `Regex`), except for
   ambiguity (`io::Error`/aliases), huge families, generated/macros, or import
   blocks that would otherwise be unwieldy.
10. Use `chromium::import!()` only in Chromium where expected.

## C. Formatting

- Use the project's formatter. `cargo fmt` is conventional in Cargo projects;
  Chromium may use `git cl format`.
- Google's preferred options include `max_width = 100`, lowercase hex digits,
  and macro matcher formatting, with an explicit allowance to omit options
  unavailable on the stable toolchain. `imports_granularity = "Module"` is
  associated with the import rule but may require nightly rustfmt; do not
  force nightly or unsupported configuration solely to set it.
- Avoid `#[rustfmt::skip]` except genuine manual alignment or generated code,
  not personal aesthetics.

## D. Ownership

- Prefer unique ownership/borrows over `Rc` and `Arc`; exceptions are valid
  when a non-shared-ownership representation would be unduly complex.
- Prefer borrow over `.clone()` when no owned copy is needed. Pragmatic clones
  can be preferable to needlessly complex lifetimes.
- Cloning `Rc`/`Arc` clones the pointer, not the underlying owned data; do not
  misdiagnose pointer clones as deep copies.

## E. Input and output contracts

- Borrow the underlying semantic data: `&T` (not `&Box<T>`), `&[T]`
  (not `&Vec<T>`), `&str` (not `&String`), `Option<&T>`
  (not `&Option<T>`). Allow justified domain-special types (e.g. ropes).
- In reusable libraries, prefer returning an iterator when it helps users
  avoid allocations or compose operations. `Vec<T>` is fine when naturally
  produced, required for indexing/ownership, or simpler for application code.
- In reusable libraries, consider taking `IntoIterator` when the function
  needs only iteration. Application code can prefer `&[T]` where simpler.
- Selectively accept `impl Into<T>` in libraries for ergonomic constructors;
  avoid extensive generic input signatures in applications.
- Use enums rather than opaque Boolean parameters; a clear `bool` remains
  acceptable when the meaning is unambiguous.
- Failure mechanism: `Option` for expected absence without useful reason;
  `Result` for recoverable failure with diagnostic context;
  `panic!` for implementation bug or broken invariant.

## F. Domain types

- Use newtypes for distinct semantic meaning, type confusion prevention,
  changeable representation, or independent API design. A type alias offers
  no additional type safety.
- Prefer named fields to positional tuple fields. Tuple structs with a single
  newtype payload and unambiguous tiny local tuples are exceptions.

## G. Traits and generic API design

- Add `impl Trait`/standard trait implementations when necessary. Low-level
  libraries may preemptively implement expected traits more than apps do.
- Prefer conventional traits such as `FromStr` over same-purpose custom
  methods. An inherent method may coexist for discovery.
- Use derive rather than handwritten impls where available.
- In libraries, favor generic parameters/`impl Trait` over `dyn Trait`, unless
  code-size tradeoffs or other concrete needs justify dynamic dispatch.
- Define traits only for actual generic programming, trait-typed boundaries,
  or useful extension methods; prefer inherent impls otherwise.
- Extension traits conventionally end with `Ext` (e.g. `ForeignTypeExt`).
- Where sensible, implement `Copy`, `Clone`, `Default`, `Eq`, `Hash`, `Ord`
  according to field semantics and API expectations. Do not derive total
  equality or ordering where domain semantics do not permit them.
- Pair important `From` implementations with discoverable constructors.
- Option-configuration structs with public fields should implement `Default`.

## H. Platform-specific concurrency

- Never bypass a poisoned mutex to retrieve protected contents under the
  referenced Google guidance.
- **[chrome]** Do not use Rust `async`/`.await` in Chromium code covered by
  this guidance. This is **NOT** a general rule for Tokio/async-std or
  ordinary Rust servers, clients, CLI tools, or services.
- **[chrome]** Do not create new threads using the Rust standard library in
  Chromium code; that environment prefers sequences.
- **[Fuchsia]** Use `fuchsia_sync::Mutex` rather than `std::sync::Mutex` or
  `parking_lot::Mutex` when covered by Fuchsia convention.
- Common Cargo application code should use its repository's tested runtime,
  synchronization, and async strategy, not transplant those restrictions.

## I. Macros, naming, and docs

- Prefer ordinary functions/types/modules to `macro_rules!` macros, then code
  generation, with procedural macros a last resort when justified.
- Use standard `as_`, `to_`, `into_` ownership conversion conventions.
- Prefer `new()`, `new_with_foo()` and `try_new()` for fallible constructors;
  `from_foo()` can accompany a corresponding `From` impl.
- Descriptive lifetimes at type/API scope are useful. Anonymous or short
  lifetimes are fine for simple local relationships; avoid needless names.
- Generic type parameter names can be compact (`T`, `U`, `K`, `V`) where the
  semantics are obvious.
- Avoid obscure abbreviations; common ecosystem terms such as `IpAddr` are OK.
- Document public interfaces with `///`, modules with `//!`. Document panic
  conditions and unsafe invariants, provide illustrative examples when useful,
  and avoid repeating what the code already says.

## J. Other features / API resilience

- Do not write `unsafe` if safe Rust suffices; isolate unavoidable unsafe,
  explain the justification and invariants using `SAFETY:` comments, and
  benchmark if performance is the only reason.
- Do not use unstable language features by default.
- Getter/setter vocabulary: `foo()`, `foo_mut()`, `take_foo()`, `set_foo()`,
  `replace_foo()`, and scoped `with_foo()`/`with_mut_foo()` when callbacks
  preserve invariants or hide a lock guard implementation detail.
- A type exposing `len()` should also expose `is_empty()`.
- Treat third-party `#[doc(hidden)]` APIs as private.
- Add `const` to public functions only with awareness that removing it is a
  breaking change for const-context callers.
- Do not overload operators for unrelated non-mathematical convenience.
- `#[non_exhaustive]` for cross-team public enums and public-field structs
  that may evolve, with exceptions for stable ontologies, controlled migrations,
  or structs that intentionally support external struct-update construction.
- `#[must_use]` for pure outcomes or values whose omission breaks semantics;
  an already-annotated return type like `Result` needs no duplicate marker.
- Avoid code that runs before `main()` (e.g. ctor-based initialization).

## K. Implementation and tests

- Annotate non-obvious type inference, especially `.parse()`, `.into()`, and
  multi-stage iterator outputs.
- Prefer `if let`/`match`/`let ... else` over `is_some()` followed by `unwrap()`.
- When a panic legitimately signifies an invariant error that cannot be
  encoded statically, prefer `expect("invariant and context")` over `unwrap()`.
- For `#[non_exhaustive]` enums, handle known variants where feasible and
  include wildcard handling for future cases.
- Add context to assertions when it helps users understand a failure.
- Prefer straightforward `for` loops over opaque iterator chains with
  branches, side effects, or complex error handling. Keep concise `any()`,
  `map()`, `filter()` expressions when clearer.
- Google first-party code prohibits `panic::catch_unwind` except very limited
  defensive boundaries, and only where unwinding actually works. Avoid it as
  an error mechanism, especially in panic-abort environments.
- Prefer `format!`/`write!` for multi-component string construction and
  literal substitution.
- Testing framework is repository-specific. Chromium uses
  `rust_gtest_interop`; Fuchsia uses `#[fuchsia::test]`. Ordinary Cargo projects
  normally use `#[test]`, not Google's platform-specific harnesses.

## L. Practical reviewer cautions

- Don't turn a **preference** into a universal prohibition. Examples:
  `Vec<T>` returns, `Arc`, cloning, dynamic dispatch, and named lifetimes
  can all be correct with sufficient context.
- Don't label every missing public `#[non_exhaustive]` a correctness bug:
  it is mainly an API-evolution choice, and documented exceptions matter.
- Don't call `async` a style violation in a regular Rust backend.
- Don't forcibly rearrange Cargo module trees to match Chromium.
- Don't mechanically add `Default` to objects that have no meaningful
  default state, or `Clone` to external resource handles.
- Distinguish a verified compiler error from a style opinion.
