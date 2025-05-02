# PLACE-CARE

Read documentation in other languages:
- [中文](README-zh.md)

# PLACE-CARE

placecare is a fast tool for searching cis-acting elements using the PLACE database.

With placecare, you can:

1. Upload sequence files to search for cis-acting elements.

2. Quickly retrieve relevant information through PLACE database's id and ac
(Data comes from the place.seq file provided by the official PLACE website)


# Installation

You can now use placecare (via Wasm) by [visiting the website](https://bio-here.github.io/placehere).

If your computer has the Rust toolchain, you can install our command-line program with the following command:

```shell
cargo install placecare
```

If you don't have the Rust toolchain installed, you can also download our compiled binary files directly from GitHub's Release:
- [Release](https://bio-here.github.io/placecare)

# Usage

First, you need to use it like this:

The core functionality of placehere is written in the `place_search` module, I/O operations are written in the `io` module,
and the `place_desc` module describes the PLACE data.

## Query Elements

We provide multiple ways to input sequences, as shown below:
```rust
use placehere::io::RecordDesc;

let input = vec![RecordDesc::new("Gh_01", "TTATAGACTCGATGGCCGCGCGG")];
let input = RecordDesc::from_file("./input.fasta");
let input = RecordDesc::from_string("\
>Gh_01
ATATCCGGATGGCATGCTGATC
");
let input = RecordDesc::from_records(bio::io::fasta::Reader::new("./input.fasta"));

let mut f = File::open("input.txt").unwrap();
let input = RecordDesc::from_reader(f);
```

Then we can search:
```rust
use placecare::place_search::Search;

// Search for a single element
let result = Search::search_element(input).unwrap();

// Search for multiple elements
let result = Search::search_elements(input).unwrap();
```

You can view the definitions in the `place_desc` module to understand the output information.

# Tips

## IUPAC Ambiguous Bases
The PLACE database uses IUPAC ambiguous base symbols ([Wikipedia](https://en.wikipedia.org/wiki/Nucleic_acid_notation)) to represent multiple possible bases.

# License
placecare is an open-source project under the MIT license. You are free to use, modify, and distribute this software, but please retain the original author's copyright notice and license information.
