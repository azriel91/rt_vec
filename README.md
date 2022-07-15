# 🗃️ rt_vec

[![Crates.io](https://img.shields.io/crates/v/rt_vec.svg)](https://crates.io/crates/rt_vec)
[![docs.rs](https://img.shields.io/docsrs/rt_vec)](https://docs.rs/rt_vec)
[![CI](https://github.com/azriel91/rt_vec/workflows/CI/badge.svg)](https://github.com/azriel91/rt_vec/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/rt_vec/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/rt_vec)

Runtime managed mutable borrowing from a vec.

This library provides a vec that allows mutable borrows to different elements at the same time. For a map implementation of this, see [`rt_map`].


## Usage

Add the following to `Cargo.toml`

```toml
rt_vec = "0.1.1" # or
rt_vec = { version = "0.1.1", features = ["unsafe_debug"] }
```

In code:

```rust
use rt_vec::RtVec;

struct A(u32);

let mut rt_vec = RtVec::new();

rt_vec.push(A(1));
rt_vec.push(A(2));

// We can validly have two mutable borrows from the `RtVec` map!
let mut a = rt_vec.borrow_mut(0);
let mut b = rt_vec.borrow_mut(1);
a.0 = 2;
b.0 = 3;

// We need to explicitly drop the A and B borrows, because they are runtime
// managed borrows, and rustc doesn't know to drop them before the immutable
// borrows after this.
drop(a);
drop(b);

// Multiple immutable borrows to the same value are valid.
let a_0 = rt_vec.borrow(0);
let _a_1 = rt_vec.borrow(0);
let b = rt_vec.borrow(1);

println!("A: {}", a_0.0);
println!("B: {}", b.0);

// Trying to mutably borrow a value that is already borrowed (immutably
// or mutably) returns `Err`.
let a_try_borrow_mut = rt_vec.try_borrow_mut(0);
let exists = if a_try_borrow_mut.is_ok() {
    "Ok(..)"
} else {
    "Err"
};
println!("a_try_borrow_mut: {}", exists); // prints "Err"
```


### Features

#### `"unsafe_debug"`

Enables the [`"unsafe_debug"`] feature of [`rt_ref`].


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[`rt_map`]: https://crates.io/crates/rt_map
[`rt_ref`]: https://crates.io/crates/rt_ref
[`"unsafe_debug"`]: https://github.com/azriel91/rt_ref#unsafe_debug
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
