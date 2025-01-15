use crate::prelude::*;

/// Table Of Contents arguments.
#[derive(Parser, Clone, Debug)]
pub struct Opt {
    /// The target file.
    #[clap(value_parser, default_value = "./README.md")]
    pub input: Input,
    /// Whether to prepend the table of contents to the file. (On by default).
    #[clap(default_value = "true")]
    pub update_readme: bool,
    /// Max heading depth for the table of contents.
    #[clap(default_value = "6")]
    pub max_depth: u32,
}
