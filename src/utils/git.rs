use crate::prelude::*;
use std::{path::Path, process::Command};

/// Utility functions that check for git information.
#[repr(transparent)]
pub struct Git();
pub type NotAGitRepository = Box<dyn std::error::Error>;

impl Git {
    const GIT: &str = "git";

    /// Check if `git` is installed.
    pub fn installed() -> bool {
        Command::new(Self::GIT)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Check if a file path is modified based on the git repository in `pwd`.
    pub fn is_file_modified<P>(f: P) -> Result<bool, NotAGitRepository>
    where
        P: AsRef<Path>,
    {
        let output = Command::new(Self::GIT)
            .args(["status", "--porcelain"])
            .arg(f.as_ref())
            .output();

        match output {
            Ok(out) if out.status.success() => {
                Ok(!String::from_utf8_lossy(&out.stdout).trim().is_empty())
            }
            Ok(_) => Ok(false),
            Err(err) => Err(Box::new(err)),
        }
    }

    /// # `--allow-dirty` checks.
    ///
    /// Run all the necessary checks for `--allow-dirty` and returning an error if the file is
    /// uncommited and `git` is installed and the flag isn't set.
    pub fn run_allow_dirty_checks<P>(args: &Opt, f: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        if !args.allow_dirty
            && Git::installed()
            && Git::is_file_modified(f).map_err(|e| e.to_string())?
        {
            return Err(format!(
                "The file has uncommited changes and the {} flag is set to false.",
                "--allow-dirty"
            )
            .into());
        }

        Ok(())
    }
}
