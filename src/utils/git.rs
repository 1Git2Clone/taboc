use crate::prelude::*;
use std::{path::Path, process::Command};

/// Utility functions that check for git information.
#[repr(transparent)]
pub struct Git();

#[derive(thiserror::Error, Debug)]
pub enum GitError {
    #[error("Not in a git repository.\n{0:?}")]
    NotInRepository(std::process::Output),
    #[error("`git` isn't installed.\n{0}")]
    NotInstalled(std::io::Error),
    #[error("The file has uncommited changes and the `--allow-dirty` flag isn't set.")]
    DisallowDirty,

    #[error("Failed to execute `git` command.\n{0:?}")]
    ExitStatusError(std::io::Error),
}

impl Git {
    const GIT: &str = "git";

    /// Check if `git` is installed.
    pub fn installed() -> Result<bool, GitError> {
        Command::new(Self::GIT)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .map_err(GitError::NotInstalled)
    }

    /// Check if a file path is modified based on the git repository in `pwd`.
    pub fn is_file_modified<P>(f: P) -> Result<bool, GitError>
    where
        P: AsRef<Path>,
    {
        let output = Command::new(Self::GIT)
            .args(["status", "--porcelain"])
            .arg(f.as_ref())
            .output()
            .map_err(GitError::ExitStatusError)?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(!stdout.trim().is_empty())
        } else {
            Err(GitError::NotInRepository(output))
        }
    }

    /// # `--allow-dirty` checks.
    ///
    /// Run all the necessary checks for `--allow-dirty` and returning an error if the file is
    /// uncommited and `git` is installed and the flag isn't set.
    pub fn run_allow_dirty_checks<P>(args: &Opt, f: P) -> Result<(), GitError>
    where
        P: AsRef<Path>,
    {
        if !args.allow_dirty && Git::installed() && Git::is_file_modified(f)? {
            return Err(format!(
                "The file has uncommited changes and the {} flag is set to false.",
                "--allow-dirty"
            )
            .into());
        }

        Ok(())
    }
}
