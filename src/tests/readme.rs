use crate::prelude::*;

#[test]
fn test_mock_input_file() -> Result<(), Error> {
    let expected = "

## Table of contents

- [Test](#test)
  - [Heading Two](#heading-two)
    - [Heading Three](#heading-three)
      - [Heading Four](#heading-four)
        - [Heading Five](#heading-five)
  - [Heading Two Number 2](#heading-two-number-2)
    - [Heading Three Number 2](#heading-three-number-2)
    - [Heading Three Number 3](#heading-three-number-3)
      - [Heading Four Number 2](#heading-four-number-2)
        - [Heading Five Number 2](#heading-five-number-2)
        - [Heading Five Number 3](#heading-five-number-3)
      - [Heading Four Number 3](#heading-four-number-3)
  - [Heading Two Number 3](#heading-two-number-3)";

    let file = std::fs::File::open(std::env::current_dir()?.join("mock_data/README.md"))?;

    assert_eq!(expected, Taboc::new(file, 6).parse()?);

    Ok(())
}
