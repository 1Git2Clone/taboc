#![allow(unused_imports)]

pub use crate::{
    app::{App, AppError},
    utils::{
        args::Opt,
        data::{TableOfContents, TabocError},
    },
};

#[cfg(feature = "git")]
pub use crate::utils::git::{Git, GitError};

pub use clap::Parser;
pub use clio::Input;
