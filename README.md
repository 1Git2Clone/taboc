# Taboc

[![Build Icon]][Build Status]&emsp;[![Docs Icon]][Docs]&emsp;[![Version Icon]][Crate]&emsp;[![License Icon]][LICENSE]

[Build Icon]: https://img.shields.io/github/actions/workflow/status/1Git2Clone/taboc/build.yml?branch=main
[Build Status]: https://github.com/1git2clone/taboc/actions?query=branch%3Amain
[Docs Icon]: https://docs.rs/taboc/badge.svg
[Docs]: https://docs.rs/taboc
[Version Icon]: https://img.shields.io/crates/v/taboc.svg
[Crate]: https://crates.io/crates/taboc
[License Icon]: https://img.shields.io/badge/license-MIT-blue.svg
[LICENSE]: LICENSE

A table of contents generator for markdown documents.

## Table of contents

- [Taboc](#taboc)
  - [Installation](#installation)
  - [Features](#features)
    - [Generic options](#generic-options)
    - [Version Control Systems](#version-control-systems)
    - [Memmap](#memmap)

## Installation

This project is available on [`crates.io`](https://crates.io/crates/taboc),
meaning the installation is as simple as typing:

```sh
cargo install taboc
```

Then you can use the `taboc` binary in your shell. Make sure to check out the
commands with the `-h` flag.

## Features

### Generic options

Related flags:

- `--input` - The input file (Default: `./README.md`).
- `--no-file-update` - Use if you want to just print the table of contents
  without updating the file.
- `--max-depth` - The maximum heading depth to search for (Default: `6`).
- `--update-existing` - Use to update the existing table of contents.

### Version Control Systems

Related flags:

- `--allow-dirty` - Checks if the `[INPUT]` file is:

  - not tracked
  - doesn't have staged changes
  - has staged changes

- `--no-vcs` - Disables VCS checks (based on the chosen VCS).

Currently supported VCS:

- [x] Git (default)
- [ ] Hg
- [ ] Pijul
- [ ] Fossil

> [!NOTE]
> If you try to use this script without a VCS then it's likely that the VCS
> checks will fail. That's why you should use the `--no-vcs` flag to explicitly
> not check for a VCS.

### Memmap

Use the operating system's memory mapping for managing the file directly.

This is space efficient, however it's often slower due to less cache locality
and also less safe in the case of multiple writers causing UB.

Refer to [`memmap2::MmapMut#safety`](https://docs.rs/memmap2/latest/memmap2/struct.MmapMut.html#safety).
