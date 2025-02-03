mod prelude;
mod utils;

use std::fs::File;

use prelude::*;

fn main() -> Result<(), Error> {
    let args = Opt::parse();
    let path = args.input.path().path();
    let Ok(file) = File::open(path) else {
        return Err(format!("Couldn't find file at: `{}`.", args.input.path()).into());
    };

    let data = TableOfContents::new(&file, args.max_depth);

    if args.no_file_update {
        println!("{}", data.parse()?);
    } else {
        data.write_to_file(path, &data.parse()?)?;
    }

    Ok(())
}
