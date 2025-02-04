#![doc = include_str!("../README.md")]
///////////////////////////////////////////////////////////////////////////////////////////////////
// Correctness
///////////////////////////////////////////////////////////////////////////////////////////////////
#![warn(rust_2018_idioms, reason = "Deprecated practices.")]
#![warn(clippy::unused_async, reason = "Remove unnecessary overhead.")]
#![warn(clippy::rc_mutex, reason = "`Rc<RefCell<T>>` is more appropriate.")]
#![warn(
    clippy::unwrap_in_result,
    reason = "Preferably map errors in order to not panic but rather return the error value gracefully."
)]
#![warn(clippy::expect_used, reason = "Prefer errors as values.")]
#![deny(clippy::unwrap_used, reason = "Prefer errors as values.")]
#![warn(
    clippy::mutex_atomic,
    reason = "Prefer using atomics where possible for less contention."
)]
#![deny(
    clippy::mutex_integer,
    reason = "Prefer using atomics where possible for less contention."
)]
#![warn(
    clippy::empty_enum_variants_with_brackets,
    reason = "Unnecessary to use."
)]
#![warn(
    clippy::redundant_pub_crate,
    reason = "Can mess up crate visibility when working with imports."
)]
#![warn(
    clippy::cloned_instead_of_copied,
    reason = "Prefer explicitly indicating types that implement `Copy` for iterators."
)]
#![deny(
    clippy::semicolon_if_nothing_returned,
    reason = "Bad practice. If nothing is returned then it's better to specify that, rather than rely on the return value of another function that also happens to return `()`."
)]
#![deny(
    clippy::redundant_clone,
    reason = "Potential performance issues and correctness."
)]
#![deny(
    clippy::used_underscore_binding,
    reason = "Underscore bindings indicate unused variables in Rust. (Unlike languages like C which use them for private variables)."
)]
#![deny(
    clippy::expl_impl_clone_on_copy,
    reason = "`Copy` is a supertrait of `Clone`."
)]
#![deny(
    clippy::clone_on_ref_ptr,
    reason = "When working with `Rc` and `Arc`, prefer explicitly cloning with `Rc::clone()` or `Arc::clone()` to indicate that the clone is a reference counter one."
)]
///////////////////////////////////////////////////////////////////////////////////////////////////
// Application specific
///////////////////////////////////////////////////////////////////////////////////////////////////
#![deny(
    clippy::infinite_loop,
    reason = "This program shouldn't directly contain any infinite loops!"
)]
///////////////////////////////////////////////////////////////////////////////////////////////////
// Conciseness
///////////////////////////////////////////////////////////////////////////////////////////////////
#![warn(clippy::struct_field_names, reason = "Unnecessary text.")]
#![warn(clippy::manual_let_else, reason = "More succinct.")]
#![warn(
    clippy::unnecessary_join,
    reason = "More succinct. Although sometimes useful to do."
)]
#![warn(clippy::redundant_closure_for_method_calls, reason = "More concise.")]
#![warn(
    clippy::if_then_some_else_none,
    reason = "Conciseness and readability."
)]
#![warn(clippy::explicit_into_iter_loop, reason = "Readability.")]
#![deny(clippy::redundant_else, reason = "Readability.")]
#![deny(clippy::string_to_string, reason = "Redundant.")]
#![deny(clippy::get_unwrap, reason = "Prefer direct indexing for conciseness.")]

/// The user-facing logic of the program.
pub mod app;

/// Crate-level re-exports
pub mod prelude;

/// Crate utilities. Can be used independently if needed.
pub mod utils;

/// Crate tests.
#[doc(hidden)]
#[cfg(test)]
mod tests;

#[doc(hidden)]
mod macros;
