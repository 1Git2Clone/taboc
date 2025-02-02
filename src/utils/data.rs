use crate::prelude::*;
use std::{
    cell::Cell,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Seek},
    os::unix::fs::FileExt,
    path::Path,
};

pub struct TableOfContents<'a> {
    file: &'a std::fs::File,
    code_block: Cell<bool>,
}

impl<'a> TableOfContents<'a> {
    const MIN_HEADING: usize = 1;
    const MAX_HEADING: usize = 6;
    const HEADING_CHAR: char = '#';
    const CODE_BLOCK_STR: &'a str = "```";
    const TOC_HEADING: &'a str = "## Table of contents";

    pub fn new(file: &'a std::fs::File) -> Self {
        Self {
            file,
            code_block: Cell::new(false),
        }
    }

    /// Make a Table of contents line based on the current heading level.
    fn make_link(heading_name: &str) -> String {
        let mut res = String::with_capacity(heading_name.len());

        for c in heading_name.chars() {
            if c == ' ' {
                res.push('-');
                continue;
            }

            if c.is_ascii() {
                res.push(c.to_ascii_lowercase());
                continue;
            }

            if c.is_uppercase() {
                res.push_str(&c.to_lowercase().to_string());
                continue;
            }

            res.push(c)
        }

        res
    }

    /// Make a Table of contents line based on the current heading level.
    fn make_line(heading_level: usize, line: &str) -> String {
        format!(
            "{}- [{}](#{})\n",
            "  ".repeat(heading_level - 1),
            line,
            Self::make_link(line)
        )
    }

    /// Check if a markdown line is valid.
    fn valid_heading(heading_level: usize, line: &str) -> bool {
        if !(Self::MIN_HEADING..=Self::MAX_HEADING).contains(&heading_level) {
            return false;
        }
        if line.len() <= heading_level || line.chars().nth(heading_level) != Some(' ') {
            return false;
        }
        if line.chars().nth(heading_level + 1).is_none() {
            return false;
        }
        true
    }

    /// We shouldn't parse headings that are in code blocks: ```.
    fn is_in_code_block(&self, line: &str) -> bool {
        if line.starts_with(Self::CODE_BLOCK_STR) {
            self.code_block.replace(!self.code_block.get());
        }
        self.code_block.get()
    }

    /// Make the table of contents based on a file.
    pub fn parse(&self) -> Result<String, Error> {
        let mut res = format!("\n\n{}\n\n", Self::TOC_HEADING);

        for l in BufReader::new(self.file).lines() {
            let line = l?;

            if self.is_in_code_block(&line) {
                continue;
            }

            let heading_count = line
                .chars()
                .take_while(|c| *c == Self::HEADING_CHAR)
                .count();

            if !Self::valid_heading(heading_count, &line) {
                continue;
            }

            let heading = line
                .chars()
                .skip(heading_count)
                .skip_while(|c| c.is_whitespace())
                .collect::<String>();

            res.push_str(&Self::make_line(heading_count, &heading));
        }

        // remove the trailing newline symbol.
        res.pop();

        Ok(res)
    }

    /// Writes to the specified path.
    ///
    /// NOTE: This ensures that there's no table of contents as the first second-level heading of a
    /// markdown document but it doesn't ensure it if it's located anywhere else.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, input: &str) -> Result<(), Error> {
        let mut target_file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut pos = 0;
        let lookup_header = "## ";
        let mut line_buf = Vec::new();
        let mut reader = BufReader::new(&target_file);
        while let Ok(char_count) = reader.read_until(b'\n', &mut line_buf) {
            println!("At line: {}", String::from_utf8_lossy(&line_buf));

            if char_count == 0 {
                break;
            }

            if line_buf.starts_with(lookup_header.as_bytes()) {
                if line_buf == Self::TOC_HEADING.as_bytes() {
                    return Err(
                        "There's already a table of contents in the start of this file.".into(),
                    );
                }
                // I wish I had an explanation for the off-by-one error here.
                pos -= lookup_header.len() as u64 - 1;
                break;
            }

            pos += char_count as u64;

            line_buf.clear();
        }

        target_file.seek(std::io::SeekFrom::Start(pos))?;
        let mut rest = Vec::<u8>::new();
        target_file.read_to_end(&mut rest)?;

        target_file.write_all_at(input.as_bytes(), pos)?;
        target_file.write_all_at(&rest, pos.checked_add(input.len() as u64).unwrap_or(pos))?;

        Ok(())
    }
}
