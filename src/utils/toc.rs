#[cfg(feature = "memmap2")]
use memmap2::MmapMut;
#[cfg(feature = "memmap2")]
use std::cmp::min;

use crate::prelude::*;
use std::{
    cell::Cell,
    fs::OpenOptions,
    io::{BufRead, BufReader, Seek, Write},
    path::Path,
};

#[cfg(not(feature = "memmap2"))]
use std::io::Read;

/// # Table of contents struct
///
/// This is the main struct responsible for reading the README.md file and parsing out the table of
/// contents with all the proper links in this format:
///
/// ```txt
/// - [Heading 1](#heading-1)
///   - [Heading 2](#heading-2)
///     - [Heading 3](#heading-3)
///     - [Heading 3 2](#heading-3-2)
///   - [Heading 2 2](#heading-2-2)
/// ```
pub struct Taboc {
    pub file: std::fs::File,
    code_block: Cell<bool>,
    max_depth: usize,
}

impl Taboc {
    const MIN_HEADING: usize = 1;
    const HEADING_CHAR: char = '#';
    const CODE_BLOCK_STR: &'static str = "```";
    const TOC_HEADING: &'static str = "## Table of contents";

    pub fn new(file: std::fs::File, max_depth: usize) -> Self {
        Self {
            file,
            code_block: Cell::new(false),
            max_depth,
        }
    }

    /// See [`RFC3986`](https://www.rfc-editor.org/rfc/rfc3986).
    fn percent_encode(c: char) -> String {
        let mut utf8_bytes: [u8; 4] = [0u8; 4];
        let bytes = c.encode_utf8(&mut utf8_bytes);
        let mut encoded = String::with_capacity(bytes.len() * 3);

        for byte in utf8_bytes {
            if byte == 0 {
                break;
            }
            encoded.push_str(&format!("%{:02X}", byte));
        }

        encoded
    }

    /// Make a Table of contents line based on the current heading level.
    fn make_link(heading_name: &str) -> String {
        let mut res = String::with_capacity(heading_name.len());

        /// Most are based on
        /// [RFC3986#section-2.2](https://www.rfc-editor.org/rfc/rfc3986#section-2.2) with some
        /// additional ones like: \`, `'`, `~`, `{` and `}`.
        ///
        /// NOTE: It's possible that not all excludable characters are covered. If you encounter a
        /// issue with a missed one then feel free to post an issue on
        /// [GitHub](https://github.com/1Git2Clone/taboc/issues).
        const IGNORED_CHARACTERS: &[char] = &[
            '+', ':', ';', '.', ',', '{', '}', '"', '@', '#', '>', '<', '[', ']', '|', '/', '?',
            '!', '$', '*', '=', '&', '\'', '(', ')', '~',
        ];

        for c in heading_name.chars() {
            if IGNORED_CHARACTERS.contains(&c) {
                continue;
            }

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

            if !c.is_alphanumeric() {
                res.push_str(&Self::percent_encode(c));
                continue;
            }

            res.push(c);
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
    fn valid_heading(&self, heading_level: usize, line: &str) -> bool {
        if !(Self::MIN_HEADING..=self.max_depth).contains(&heading_level) {
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

        for l in BufReader::new(&self.file).lines() {
            let line = l?;

            if self.is_in_code_block(&line) {
                continue;
            }

            let heading_count = line
                .chars()
                .take_while(|c| *c == Self::HEADING_CHAR)
                .count();

            if !self.valid_heading(heading_count, &line) {
                continue;
            }

            if line.starts_with(Self::TOC_HEADING) {
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
    pub fn write_to_file<P: AsRef<Path>>(
        &self,
        path: P,
        input: &str,
        update_existing: bool,
    ) -> Result<(), Error> {
        let mut target_file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut pos = 0;
        let lookup_header = "## ";
        let mut line_buf = Vec::new();
        let mut reader = BufReader::new(&target_file);

        let mut already_exists = false;

        while let Ok(char_count) = reader.read_until(b'\n', &mut line_buf) {
            if char_count == 0 {
                break;
            }

            if line_buf.starts_with(lookup_header.as_bytes()) {
                let windows_toc = line_buf[line_buf.len().saturating_sub(2)] != b'\r'
                    && &line_buf[0..line_buf.len().saturating_sub(1)]
                        == Self::TOC_HEADING.as_bytes();
                let unix_toc = &line_buf[0..line_buf.len()] == Self::TOC_HEADING.as_bytes();
                if !update_existing && (windows_toc || unix_toc) {
                    return Err(
                        anyhow!("There's already a table of contents in the first heading of the second level of this file.")
                    );
                } else if windows_toc || unix_toc {
                    already_exists = true;
                }
                // I wish I had an explanation for the off-by-one error here.
                pos -= lookup_header.len() as u64 - 1;
                break;
            }

            pos += char_count as u64;

            line_buf.clear();
        }

        target_file.seek(std::io::SeekFrom::Start(pos))?;
        #[cfg(feature = "memmap2")]
        let rest_map = unsafe { MmapMut::map_mut(&target_file)? };
        #[cfg(feature = "memmap2")]
        let mut rest = &rest_map[..];
        #[cfg(not(feature = "memmap2"))]
        let mut rest = Vec::<u8>::new();
        #[cfg(not(feature = "memmap2"))]
        target_file.read_to_end(&mut rest)?;

        target_file.seek(std::io::SeekFrom::Start(pos))?;

        if already_exists {
            let mut reader = BufReader::new(&target_file);
            let mut drain_pos = 0;

            // reads [`Self::TOC_HEADING`] twice. Log each line to understand why it's like this.
            let mut end_heading_count = 3;
            let mut last_line_char_count = 0;

            while let Ok(char_count) = reader.read_until(b'\n', &mut line_buf) {
                // println!("LINE: {}", String::from_utf8_lossy(&line_buf));

                if line_buf.starts_with(b"#") {
                    end_heading_count -= 1;
                } else if end_heading_count == 0 {
                    drain_pos -= last_line_char_count;
                    break;
                }
                if line_buf.trim_ascii().is_empty() && end_heading_count == 1 {
                    drain_pos -= char_count;
                }

                drain_pos += char_count;

                line_buf.clear();
                last_line_char_count = char_count;
            }

            #[cfg(feature = "memmap2")]
            {
                rest = &rest_map[min(drain_pos - 1, rest_map.len() - 1)..];
            }
            #[cfg(not(feature = "memmap2"))]
            rest.drain(..drain_pos);
        }

        target_file.seek(std::io::SeekFrom::Start(pos))?;
        target_file.write_all(input.as_bytes())?;
        #[cfg(feature = "memmap2")]
        target_file.write_all(rest)?;
        #[cfg(feature = "memmap2")]
        rest_map.flush()?;
        #[cfg(not(feature = "memmap2"))]
        target_file.write_all(&rest)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Taboc;

    #[test]
    fn percent_encode() {
        assert_eq!(Taboc::percent_encode('😁'), "%F0%9F%98%81");
        assert_eq!(Taboc::percent_encode('♊'), "%E2%99%8A");
        assert_eq!(Taboc::percent_encode('⏳'), "%E2%8F%B3");
        assert_eq!(Taboc::percent_encode('❌'), "%E2%9D%8C");
        assert_eq!(Taboc::percent_encode('⏪'), "%E2%8F%AA");
        assert_eq!(Taboc::percent_encode('⛪'), "%E2%9B%AA");
        assert_eq!(Taboc::percent_encode('⟣'), "%E2%9F%A3");
        assert_eq!(Taboc::percent_encode('⛟'), "%E2%9B%9F");
    }
}
