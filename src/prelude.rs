#![allow(unused_imports, reason = "Potential external usage.")]
/*!
Re-exports for potential external use.

WARN: This re-exports potentially ambiguous types like [`anyhow::Error`], [`clap::Parser`] and
[`crate::app::App`].
*/

pub use crate::{
    app::App,
    utils::{args::Opt, toc::Taboc},
};

#[cfg(feature = "git")]
pub use crate::utils::git::Git;

pub use clap::Parser;
pub use clio::Input;

pub use anyhow::{anyhow, Error};
