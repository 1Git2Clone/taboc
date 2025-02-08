use crate::prelude::*;
use clap::ArgAction;

/// A table of contents generator for markdown documents written in Rust.
#[derive(Parser, Clone, Debug)]
pub struct Opt {
    /// The target file.
    #[clap(value_parser, default_value = "./README.md")]
    pub input: Input,
    /// Use to override the existing table of contents in [INPUT].
    #[clap(long, action=ArgAction::SetTrue)]
    pub update_existing: bool,
    /// Use to only print the table of contents to stdout. If not set it'll try to update the file
    /// directly.
    #[clap(long, action=ArgAction::SetTrue)]
    pub no_file_update: bool,
    /// Use to not consider uncommited git changes.
    #[cfg(feature = "git")]
    #[clap(long, action=ArgAction::SetTrue)]
    pub allow_dirty: bool,
    /// Don't check for changes on your Version Control System.
    #[cfg(feature = "git")]
    #[clap(long, action=ArgAction::SetTrue)]
    pub no_vcs: bool,
    /// Max heading depth for the table of contents.
    #[clap(long, default_value = "6")]
    pub max_depth: usize,
}
