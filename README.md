# False Positive for rust-lang/rust#83583
The deprecation lint `proc_macro_derive_resolution_fallback` is intended to catch proc macro generated code that refers to items from parent modules that should not be in scope:

```rust
struct Outer {}

mod generated {
    struct Dummy {
        inner: Outer,
    }
}
```

The lint works for this case, and a proc macro can add a `use super::*;` line to make this code valid:

```rust
struct Outer {}

mod generated {
    use super::*;

    struct Dummy {
        inner: Outer,
    }
}
```

However, if the same corrected proc macro runs inside a `fn` body the lint will still be triggered, incorrectly:

```rust
fn item() {
    struct Inner {}

    mod generated {
        use super::*;

        struct Dummy {
            inner: Inner,
        }
    }
}
```

```
warning: cannot find type `Inner` in this scope
  --> src\lib.rs:12:12
   |
12 |     struct Inner {}
   |            ^^^^^ names from parent modules are not accessible without an explicit import
   |
   = note: `#[warn(proc_macro_derive_resolution_fallback)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
```

See `foo-derive/src/lib.rs` for the proc macro implementation and `src/lib.rs` for usage. This warning can be reproduced with

```sh
cargo check
```