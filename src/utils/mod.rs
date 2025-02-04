/// User-input arguments handled via the `clap` and `clio` crates.
pub mod args;

/// The main logic in this applcation.
pub mod toc;

/// Utilities for checking if the changes from the args are commited or not.
#[cfg(feature = "git")]
pub mod git;
