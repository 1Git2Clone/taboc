/*!
Mainly made for benchmarking `taboc`.

The performance (Ryzen 5 3600X):

```sh
time taboc output.md --allow-dirty
# taboc output.md --allow-dirty  0.89s user 0.40s system 97% cpu 1.321 total
```
*/

use rand::{
    distr::{Alphabetic, SampleString},
    prelude::*,
};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

const RAND_CHAR_COUNT: usize = 128;
const LINE_COUNT: usize = 1_000_000;

fn generate_heading_level(rng: &mut ThreadRng) -> u32 {
    rng.next_u32() % 6
}

fn generate_heading(rng: &mut ThreadRng) -> String {
    Alphabetic.sample_string(rng, RAND_CHAR_COUNT)
}

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open("output.md")?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::rng();

    // Taboc requires the first two headings to be proper markdown headings.
    // (Otherwise it'd just insert itself in a wrong section).
    writer.write_all(format!("# {}\n\n", generate_heading(&mut rng)).as_bytes())?;
    writer.write_all(format!("## {}\n\n", generate_heading(&mut rng)).as_bytes())?;

    for _ in 3..=LINE_COUNT {
        let line = format!(
            "{} {}\n\n",
            "#".repeat(generate_heading_level(&mut rng) as usize + 1),
            generate_heading(&mut rng)
        );
        writer.write_all(line.as_bytes())?;
    }

    Ok(())
}
