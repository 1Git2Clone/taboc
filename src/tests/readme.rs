use crate::prelude::*;

#[test]
fn test_mock_input_file() -> Result<(), Error> {
    let expected = "## Table of contents

- [Test](#test)
  - [Heading Two](#heading-two)
    - [Heading Three](#heading-three)
      - [Heading Four](#heading-four)
        - [Heading Five](#heading-five)
  - [Heading Two #2](#heading-two-#2)
    - [Heading Three #2](#heading-three-#2)
    - [Heading Three #3](#heading-three-#3)
      - [Heading Four #2](#heading-four-#2)
        - [Heading Five #2](#heading-five-#2)
        - [Heading Five #3](#heading-five-#3)
      - [Heading Four #3](#heading-four-#3)
  - [Heading Two #3](#heading-two-#3)

";

    let file = std::fs::File::open(std::env::current_dir()?.join("mock_data/README.md"))?;

    assert_eq!(expected, TableOfContents::new(&file).parse()?);

    Ok(())
}
