//! Runtime managed mutable borrowing from a vec.
//!
//! This library provides a vec that allows mutable borrows to different
//! elements at the same time. For a map implementation of this, see [`rt_map`].
//!
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! rt_vec = "0.1.0"
//! ```
//!
//! In code:
//!
//! ```rust
//! use rt_vec::RtVec;
//!
//! struct A(u32);
//!
//! let mut rt_vec = RtVec::new();
//!
//! rt_vec.push(A(1));
//! rt_vec.push(A(2));
//!
//! // We can validly have two mutable borrows from the `RtVec` map!
//! let mut a = rt_vec.borrow_mut(0);
//! let mut b = rt_vec.borrow_mut(1);
//! a.0 = 2;
//! b.0 = 3;
//!
//! // We need to explicitly drop the A and B borrows, because they are runtime
//! // managed borrows, and rustc doesn't know to drop them before the immutable
//! // borrows after this.
//! drop(a);
//! drop(b);
//!
//! // Multiple immutable borrows to the same value are valid.
//! let a_0 = rt_vec.borrow(0);
//! let _a_1 = rt_vec.borrow(0);
//! let b = rt_vec.borrow(1);
//!
//! println!("A: {}", a_0.0);
//! println!("B: {}", b.0);
//!
//! // Trying to mutably borrow a value that is already borrowed (immutably
//! // or mutably) returns `Err`.
//! let a_try_borrow_mut = rt_vec.try_borrow_mut(0);
//! let exists = if a_try_borrow_mut.is_ok() {
//!     "Ok(..)"
//! } else {
//!     "Err"
//! };
//! println!("a_try_borrow_mut: {}", exists); // prints "Err"
//! ```
//!
//! [`rt_map`]: https://crates.io/crates/rt_map

// Re-exports
pub use rt_ref::{BorrowFail, Cell, CellRef, CellRefMut, Ref, RefMut};

pub use crate::rt_vec::RtVec;

mod rt_vec;