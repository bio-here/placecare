//!
//! # Placecare
//!
//! Im a fast toolkit to search for cis-acting
//! regulatory elements using the PLACE database.
//!
//! This is the library crate for placecare,
//! which provides the core functionality for searching and querying the PLACE database.
//!
//! # Usage
//!
//! ## Searching for elements
//!
//! We provide multiple ways to input sequences, as shown below:
//! ```rust
//! use biohere_placehere::io::RecordDesc;
//!
//! let input = vec![RecordDesc::new("Gh_01", "TTATAGACTCGATGGCCGCGCGG")];
//! let input = RecordDesc::from_file("./input.fasta");
//! let input = RecordDesc::from_string("\
//! >Gh_01
//! ATATCCGGATGGCATGCTGATC
//! ");
//! let input = RecordDesc::from_records(bio::io::fasta::Reader::new("./input.fasta"));
//!
//! let mut f = File::open("input.txt").unwrap();
//! let input = RecordDesc::from_reader(f);
//! ```
//!
//! Then we can search:
//! ```rust
//! use biohere_placecare::place_search::Search;
//!
//! // Search for a single element
//! let result = Search::search_elements_single_seq(input).unwrap();
//!
//! // Search for multiple elements
//! let result = Search::search_elements(input).unwrap();
//! ```
//!
//! ## Query
//!
//! We can query the PLACE databse using the following methods:
//!
//! ```rust
//! use biohere_placecare::search::Search;
//!
//! // The function will return a vector of Option<SeqDesc>
//! // for which is a result of the input sequence.
//! let e1: Vec<Option<SeqDesc>> = query_elements_by_id(&vec!["TATABOX1", "TATABOX2"]);
//! let e2: Vec<Option<SeqDesc>> = query_elements_by_ac(&vec!["S000023", "S000260"]);
//! ```

/// Description structure of PLACE database.
pub mod place_desc;

/// I/O Format
pub mod io;

/// Ways to search PLACE database.
pub mod place_search;

/// Maintains the PLACE database.
pub mod db;
