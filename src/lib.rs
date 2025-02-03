#![doc = include_str!("../README.md")]

/// Crate-level re-exports
pub mod prelude;

/// Crate utilities. Can be used independently if needed.
pub mod utils;

/// Crate tests.
#[doc(hidden)]
#[cfg(test)]
mod tests;
