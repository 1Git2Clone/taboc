/*!
The utilities consist of CLI parsing and Table of contents (TOC) generation.

The CLI parsing is handled entirely by the [`clap`] crate in [`args::Opt`] whilst the TOC
generation is handled via [`toc::Taboc`].

## Version Control System

Currently supported VCS:

- [`git`] via the [`git::Git`] struct.

NOTE: This could undergo future changes and the [`git`] module could end up being a part of a
`vcs/` subdirectory from here.
*/

/// User-input arguments handled via the `clap` and `clio` crates.
pub mod args;

/// The main logic in this applcation.
pub mod toc;

/// Utilities for checking if the changes from the args are commited or not.
#[cfg(feature = "git")]
pub mod git;
