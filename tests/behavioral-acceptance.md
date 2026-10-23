# Behavioral acceptance for maintainers

Run these scenarios in isolated temporary projects with the complete installed
skill folder. Give the agent the request and raw fixture, without the expected
judgment. Keep the resulting diff and actual check output for assessment; mark
unrun scenarios unverified. These are reusable evaluation cases, not a claim
that the current skill has passed them.

| Scenario and fixture | Request | Observable acceptance |
| --- | --- | --- |
| Ordinary Cargo library, existing MSRV and module layout | Add a small fallible parser API and tests | Preserves edition/MSRV/layout; rejects malformed and boundary input without routine panic; no Chromium-only async/layout prohibition or speculative framework |
| Existing async runtime; task updates local state before awaiting a simulated response; caller can cancel | Fix cancellation leaving the state stuck and make shutdown complete | Identifies partial effects and task/resource owner; deterministic regression exercises cancellation after progress and shutdown; does not assume handle drop or async `Drop` solves cleanup |
| Safe wrapper over an FFI buffer/callback; foreign allocator and callback thread are documented; safe API permits invalid length | Repair the wrapper while preserving the foreign contract | Establishes a behavior/safety regression where feasible; validates before constructing Rust references; preserves allocation pairing and callback lifetime/thread policy; does not invent `Send`/`Sync` or silently change ABI |
| Library CI declares mutually exclusive features, an MSRV and an embedded `no_std` target; local target unavailable | Validate a feature-gated API change | Selects supported affected combinations; distinguishes host/default success from `no_std`/cross-target evidence; reports unavailable checks; does not impose `--all-features`, install globally or raise MSRV |
| Small reproducible off-by-one defect with direct callers and an existing test suite | Fix the boundary rejection and verify it | Regression fails for the actual boundary before repair, passes afterward; bounded scan checks same cause; final checks follow final edits; unrelated setup failure is not called RED |

For portability, copy `SKILL.md`, all `references/`, and `scripts/` together to a
temporary installation matching each client's documented discovery scope.
Invoke the installed skill from a different project working directory and run a
relevant scenario. Verify the agent actually reads a conditional reference and
resolves the helper relative to the skill installation. Also verify an ordinary
parser task does not load unrelated unsafe/async references. Report explicit
invocation and automatic discovery separately. Clean up temporary installations;
do not overwrite an existing user installation for an evaluation.

A frontmatter/link check is structural evidence only. Successful client loading
is discovery evidence; the scenario's output and checks are behavioral evidence.
Record unavailable clients and unsupported platforms rather than extrapolating.

## Executable boundary fixtures

Run `python3 tests/run-rust-scenarios.py` from this checkout. Requires Python 3,
Cargo, rustfmt, Clippy, and a host C11 compiler (`cc`) and archiver (`ar`). It
copies the fixture into a temporary directory, uses offline Cargo commands,
and removes the directory even on failure; it does not install tools or targets.
It is currently a macOS/Linux fixture, not a Windows portability claim.

The fixture owns the port range (1–65535), a caller-owned cancellable future,
and a four-byte C allocation with a matching deallocator and no callbacks.
The runner injects a boundary rejection and missing cancellation cleanup into
separate temporary copies, checks the actual assertion failures, restores the
correct implementation, and runs runtime tests and compile-fail API examples.
It also checks host `no_std` configurations separately and confirms that the
intentionally incompatible feature pair fails for its declared contract.
The C build is real foreign integration, not a mock of the allocator boundary.

These checks exercise the Rust mechanisms used in the evaluation cases. They
**do not** establish that an AI client followed the skill. The manually polled
future demonstrates cancellation and caller-owned cleanup; it does not validate
spawned-runtime shutdown. Callback lifecycle, embedded execution, the declared
MSRV, Miri and exhaustive concurrency remain separate checks.
See [client validation](client-validation.md) for observed discovery and inference
limits; never promote discovery-only results to behavioral acceptance.
