pub type Error = Box<dyn ::core::error::Error + ::core::marker::Send + ::core::marker::Sync>;

pub use crate::utils::{args::Opt, data::TableOfContents};

pub use clap::Parser;
pub use clio::Input;
