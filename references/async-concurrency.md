# Async and concurrency boundaries

This is skill-added engineering guidance, not a Google platform rule. Read
when changing async work, shared state, cancellation, or shutdown. Preserve the
project's runtime and synchronization choices unless the task requires a change.

## Cancellation and partial effects

- Identify who owns each future and which operation can drop it: a timeout,
  selection branch, caller cancellation, or shutdown. Inspect the runtime/API
  contract; dropping a handle need not cancel the underlying work.
- At cancellation points, inspect state already changed and external effects
  already issued. Dropping a future does not roll back a write or undo a remote
  request. Preserve invariants through a committed transition, explicit recovery,
  or another mechanism justified by the protocol.
- For retryable operations, establish whether partial progress can repeat an
  effect. Reuse existing transaction/idempotency mechanisms where relevant;
  do not invent them for an operation with no such risk.
- `Drop::drop` is synchronous. Do not rely on it awaiting a network flush or
  asynchronous close. Give required async cleanup an explicit completion/shutdown
  path, with a synchronous fallback appropriate to the owned resource. Do not
  assume destructors run on process abort or every process exit path.

## Tasks, locks, and thread boundaries

- Name the owner responsible for observing task errors, requesting shutdown,
  and waiting for completion when required. Inspect detached work and blocking
  work separately: cancellation of the awaiting future may leave them running.
- Stop accepting work, signal outstanding work, and drain or terminate it as the
  existing protocol requires. Define what a timeout leaves behind and who owns
  remaining resources; avoid indefinite waits or silently lost failures.
- Inspect guards held across `await`. A synchronous lock can block executor
  threads; even an async lock can create deadlock, reentrancy, or excessive
  serialization. Narrow the guarded state transition or justify holding it
  across suspension from actual lock ordering and callers.
- Derive `Send`/`Sync` requirements from the executor, captured fields, and
  callbacks. `Arc<T>` gives shared ownership, not thread safety for arbitrary
  `T`. Do not add unsafe implementations just to satisfy a spawn constraint.

## Focused proof

Exercise relevant suspension boundaries, cancellation after partial progress,
shutdown with outstanding work, failure observation, and resource release.
Use controlled synchronization instead of timing-only sleeps where practical.
One successful execution does not establish every possible schedule. Read
[validation.md](validation.md) for risk-specific tools and evidence limits.

Authoritative contracts to consult for the actual API in use:
[Future](https://doc.rust-lang.org/std/future/trait.Future.html),
[Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html), and
[Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html).
