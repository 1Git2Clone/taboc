"""
Mainly made for benchmarking `taboc`.

The performance (Ryzen 5 3600X):

```sh
time taboc output.md --allow-dirty
# taboc output.md --allow-dirty  0.89s user 0.40s system 97% cpu 1.321 total
```
"""

from pathlib import Path
from collections.abc import Iterable
import string
import random


PROJECT_ROOT = Path(__file__).parent.parent
OUTPUT_FILENAME: str = "output.md"
WARNING: str = "".join(
    [
        "WARNING: For large line counts this script takes a while to run.\n",
        "For comparison: ~10 seconds for `LINE_COUNT` = 1,000,000 ",
        "on my Ryzen 5 3600X system.",
    ]
)
LINE_COUNT: int = 1_000_000
RAND_CHAR_COUNT: int = 128
OUTPUT_PATH: Path = PROJECT_ROOT.joinpath(OUTPUT_FILENAME)


def rand_chars(n: int) -> str:
    return "".join(random.choices(string.ascii_letters, k=n))


def generate_md_lines(
    n: int = LINE_COUNT, rand_char_count: int = RAND_CHAR_COUNT
) -> Iterable[str]:
    for i in range(n):
        # Taboc requires the first two headings to be proper markdown headings.
        # (Otherwise it'd just insert itself in a wrong section).
        if i == 0:
            yield f"# {rand_chars(rand_char_count)}\n\n"
            continue
        if i == 1:
            yield f"## {rand_chars(rand_char_count)}\n\n"
            continue

        yield f"{'#' * random.randint(1, 6)} {rand_chars(rand_char_count)}\n\n"


def main():
    if LINE_COUNT > 50_000:
        print(WARNING)

    with open(OUTPUT_PATH, "w") as f:
        f.writelines(generate_md_lines())


if __name__ == "__main__":
    main()
