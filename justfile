add-year year:
  cargo new --bin "aoc-{{year}}"
  cargo add --package "aoc-{{year}}" aoc-utils
