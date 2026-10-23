# Unsafe and foreign boundaries

This is skill-added engineering guidance, not a Google platform rule. Read for
unsafe operations, unsafe traits, pinning, or foreign interfaces. Keep contracts
at the abstraction owning the invariant; comments at a call site cannot repair
an unsound safe API.

## Unsafe abstractions

- First establish why safe Rust or an existing sound abstraction is insufficient.
  Identify every safe constructor and mutation path capable of reaching unsafe
  code, including error paths and destruction.
- State the applicable validity obligations: bounds, alignment, initialized
  values, valid representations, pointer provenance, aliasing, lifetime,
  mutation authority, and thread access. A pointer being non-null is insufficient.
- A safe wrapper must establish these preconditions for every accepted input.
  If a caller must uphold an unchecked safety obligation, expose/document an
  appropriate unsafe contract rather than disguising it as a safe API.
- Put a local `SAFETY:` justification next to the operation, explaining the
  evidence that discharges each relevant obligation. Document unsafe public
  caller obligations in a `# Safety` section. Do not merely restate the code.
- For pinning, check address stability, projection, replacement, and destruction
  obligations. `Pin` is not a general ban on moving all values: distinguish
  `Unpin` types and uphold the pinning contract for pinned data until drop.
- For `unsafe impl Send`/`Sync`, justify all stored fields, interior mutation,
  ownership transfer, callback access, destruction thread, and foreign thread
  constraints. Synchronizing one operation is not proof for the whole type.

## FFI contracts

- Match the actual ABI and representation. `repr(C)` helps define layout but
  does not make Rust references, `String`, `Vec`, or arbitrary enum values valid
  foreign inputs. Validate lengths, discriminants, nullability, encoding, and
  buffer validity before constructing Rust values or references.
- Pair allocation and deallocation under the agreed allocator/API. State who
  owns buffers and handles, whether ownership transfers, when borrows end,
  and which side may free or mutate the resource. Check error cleanup too.
- Establish callback registration, captured-data lifetime, deregistration and
  in-flight callback completion. Include thread affinity, concurrent callbacks,
  reentrancy, and destruction ordering in the contract.
- Establish panic/unwind policy on both sides and the chosen ABI. Do not allow
  unwinding through a boundary that cannot support it. `catch_unwind` catches
  only unwinding panics, not aborts; use it only where the project's boundary
  policy permits defensive containment, with an explicit error translation.
  Do not use it as routine recoverable error handling.
- If bindings are generated, edit their owning declaration/generator and
  regenerate using the repository workflow; review both sides of the boundary.

## Focused proof

Exercise valid and rejected inputs, boundary lengths, ownership transfer,
callback shutdown, and failure cleanup through the real wrapper. Foreign callers
and supported targets may require integration checks. Compiler acceptance and a
clean Miri run do not prove the entire FFI contract or unsafe soundness; see
[validation.md](validation.md).

Authoritative contracts:
[Rust undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html),
[Pin](https://doc.rust-lang.org/std/pin/),
[Send and Sync](https://doc.rust-lang.org/nomicon/send-and-sync.html), and
[FFI and unwinding](https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding).
