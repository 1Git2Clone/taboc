/*!
Setting up the application is as simple as:

```rust
use taboc::prelude::*;

fn main() -> Result<(), Error> {
    // The error assert is done because the project already has a table of contents.
    assert!(App::init()?.run().is_err());

    Ok(())
}
```

Alternatively, you can also make the app from existing args too:

```rust
use taboc::prelude::*;

fn main() -> Result<(), Error> {
    let args = Opt::parse();

    // The error assert is done because the project already has a table of contents.
    assert!(App::from_args(&args)?.run().is_err());

    Ok(())
}
```
*/

use std::{borrow::Cow, fs::File, path::PathBuf};

use crate::prelude::*;

pub struct App<'a> {
    pub args: Cow<'a, Opt>,
    pub path: PathBuf,
    pub taboc: Taboc,
}

impl<'a> App<'a> {
    /// Make the `App` struct.
    pub fn init() -> Result<Self, Error> {
        let args: Cow<'a, Opt> = Cow::Owned(Opt::parse());
        let path = args.input.path().path().to_path_buf();
        let taboc = Taboc::new(File::open(&path)?, args.max_depth);

        Ok(Self { args, path, taboc })
    }

    /// Make the `App` struct from pre-existing `args`.
    #[allow(dead_code)]
    pub fn from_args(args: &'a Opt) -> Result<Self, Error> {
        let path = args.input.path().path().to_path_buf();
        let table_of_contents = Taboc::new(File::open(&path)?, args.max_depth);

        Ok(Self {
            args: Cow::Borrowed(args),
            path,
            taboc: table_of_contents,
        })
    }

    /// Run the main application logic.
    pub fn run(&self) -> Result<(), Error> {
        if self.args.no_file_update {
            println!("{}", self.taboc.parse()?);
            return Ok(());
        }

        #[cfg(feature = "git")]
        {
            Git::run_allow_dirty_checks(&self.args, &self.path)?;
        }

        self.taboc.write_to_file(&self.path, &self.taboc.parse()?)?;

        Ok(())
    }
}
