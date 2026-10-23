# Idiomatic Rust examples for this Skill

These are illustrative examples. Adopt only patterns relevant to the task,
with the repository's own error types, names, and abstractions.

## 1. Flexible borrowed parameters

Avoid a container-specific borrowed argument:

```rust
fn is_relevant(names: &Vec<String>, needle: &String) -> bool {
    names.iter().any(|name| name == needle)
}
```

Prefer semantic borrows:

```rust
fn is_relevant(names: &[String], needle: &str) -> bool {
    names.iter().any(|name| name.as_str() == needle)
}
```

This accepts vectors, slices, and string literals without materializing a new
owned `String` just to make the call.

## 2. Match instead of check-then-unwrap

Avoid:

```rust
fn first_name(names: &[String]) -> Option<&str> {
    if !names.is_empty() {
        return Some(names.first().unwrap());
    }
    None
}
```

Prefer:

```rust
fn first_name(names: &[String]) -> Option<&str> {
    names.first().map(String::as_str)
}
```

For control-flow-heavy cases, `if let Some(value) = ...` is equally valid.

## 3. Semantically clear API flags

Avoid unclear call sites:

```rust
fn load_records(force_refresh: bool) {}
load_records(true);
```

Prefer:

```rust
#[derive(Clone, Copy, Debug)]
enum CachePolicy {
    UseCache,
    Refresh,
}

fn load_records(policy: CachePolicy) {}
load_records(CachePolicy::Refresh);
```

## 4. Domain newtypes and constructor naming

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AccountId(u64);

impl AccountId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}
```

Newtypes prevent accidentally passing one distinct integer identifier where
another is expected. Use a fallible `try_new` when validation is required.

## 5. Error semantics

```rust
use std::num::ParseIntError;

// Caller can recover from an invalid number and needs the reason.
fn parse_quantity(input: &str) -> Result<u32, ParseIntError> {
    input.parse()
}

// Missing element is ordinary absence, not a diagnostic failure.
fn find_first_even(values: &[u32]) -> Option<u32> {
    values.iter().copied().find(|value| value % 2 == 0)
}
```

Use panics only for broken internal invariants. For process boundaries
(e.g. network input), return errors rather than panicking on invalid input.

## 6. Inherent method rather than a purely organizational trait

Prefer:

```rust
struct Counter {
    value: usize,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}
```

Do not invent `trait CounterActions` solely to hold `increment` when no
polymorphic use or extension boundary exists.

## 7. Explicit control flow over side-effectful iterator chains

```rust
fn parse_valid_ids(inputs: &[String]) -> (Vec<u64>, usize) {
    let mut ids = Vec::new();
    let mut errors = 0;

    for input in inputs {
        match input.parse::<u64>() {
            Ok(id) => ids.push(id),
            Err(_) => errors += 1,
        }
    }

    (ids, errors)
}
```

For a pure predicate, keep `values.iter().any(...)`: avoid rewriting clear
combinators into verbose loops without a reason.

## 8. Imports: group by originating module

```rust
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
```

Do not nest a different module path inside braces, for example:
`use std::{collections::HashMap, path::Path};`.

## 9. Public API documentation and panic contracts

```rust
/// Returns a borrowed account name by index.
///
/// # Panics
///
/// Panics if `index` is outside the account list.
pub fn account_name(accounts: &[String], index: usize) -> &str {
    accounts[index].as_str()
}
```

Use this indexing contract only where an out-of-bounds index really is a caller
programming error; otherwise return `Option<&str>`.

## 10. Unsafe requires a local invariant explanation

```rust
// Prefer this safe implementation when its semantics are sufficient.
fn last_value(values: &[u64]) -> Option<u64> {
    values.last().copied()
}
```

When an `unsafe` block truly cannot be avoided, document its safety conditions
at the call site; do not add unsafe for speculative micro-optimizations.

## 11. `Arc` is situational, not banned in general Rust

For an ordinary Tokio or multi-threaded Cargo service, sharing an immutable,
expensive-to-clone object can justify `Arc<T>`; first assess whether scoped
borrowing or explicit ownership transfer would be simpler. Chromium has
separate concurrency rules; consult the platform-specific section.
