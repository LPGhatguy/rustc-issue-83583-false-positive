#![allow(dead_code)]

use foo_derive::Foo;

// This case is fine; no warnings
#[derive(Foo)]
struct Outer {}

// This case results in a false positive warning
fn item() {
    #[derive(Foo)]
    struct Inner {}
}
