//!
//! The IO Module of the DB.

use bio::io::fasta::Records;
use std::fmt::Debug;

/// The structure is used to describe the input query sequence.
#[derive(Debug, Clone)]
pub struct RecordDesc {
    id: String,  // input query sequence id
    seq: String, // input query sequence
    len: usize,  // input query sequence length
}

impl RecordDesc {
    /// create a new RecordDesc.
    pub fn new(id: &str, seq: &str) -> Self {
        Self {
            id: id.to_owned(),
            seq: seq.to_string().to_uppercase(),
            len: seq.len(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn seq(&self) -> &str {
        &self.seq
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// create new RecordDescs from fasta records of [bio] crate.
    pub fn from_records<B>(records: Records<B>) -> Vec<Self>
    where
        B: std::io::BufRead + Debug,
    {
        let mut res = vec![];
        for record in records {
            let record = record.expect("Error<bio>: Failed to read record");
            let id = record.id();
            let seq = record.seq().to_owned();
            let seq = std::str::from_utf8(&seq).expect("Error<bio>: Invalid UTF-8");

            res.push(Self::new(id, &seq));
        }
        res
    }

    /// create new RecordDescs from a Reader.
    pub fn from_reader<R>(reader: R) -> Vec<Self>
    where
        R: std::io::Read + Debug,
    {
        let reader = bio::io::fasta::Reader::new(reader);
        Self::from_records(reader.records())
    }

    /// create new RecordDescs from a file.
    pub fn from_file<P>(reader: P) -> Vec<Self>
    where
        P: AsRef<std::path::Path> + std::fmt::Display + Debug,
    {
        let reader =
            bio::io::fasta::Reader::from_file(reader).expect("Error<bio>: Failed to read file");
        Self::from_records(reader.records())
    }

    /// create new RecordDescs from a string.
    pub fn from_string<P>(string: P) -> Vec<Self>
    where
        P: AsRef<str> + std::fmt::Display,
    {
        let reader = bio::io::fasta::Reader::new(string.as_ref().as_bytes());
        Self::from_records(reader.records())
    }
}

/// The structure is used to describe the search result.
#[derive(Debug, Clone)]
pub struct SearchResult<'a> {
    pub id: String,
    pub count: usize,
    pub search_descs: Vec<SearchedDesc<'a>>,
}

impl<'a> SearchResult<'a> {
    pub fn new(id: &str, searched_descs: Vec<SearchedDesc<'a>>) -> Self {
        Self {
            id: id.to_owned(),
            count: searched_descs.len(),
            search_descs: searched_descs,
        }
    }

    pub fn sort_self(&mut self) {
        self.search_descs.sort_by(|a, b| a.q_start.cmp(&b.q_start));
    }
}

// TODO： usize -> String -> str
/// The structure is used to describe the searched element in the database.
#[derive(Clone, Debug)]
pub struct SearchedDesc<'a> {
    pub q_id: &'a str,   // input query sequence id
    pub q_start: usize,  // start position of the query
    pub q_end: usize,    // end position of the query
    pub q_dir: usize,    // direction of the sequence, 0 for - and 1 for +
    pub e_id: &'a str,   // element id
    pub e_len: usize,    // element length
    pub e_sq: &'a str,   // element sequence
    pub e_ac: &'a str,   // element accession number
    pub e_desc: &'a str, // element description
}

impl<'a> SearchedDesc<'a> {
    pub fn new(
        q_id: &'a str,
        q_start: usize,
        q_end: usize,
        q_dir: usize,
        e_id: &'a str,
        e_len: usize,
        e_sq: &'a str,
        e_ac: &'a str,
        e_desc: &'a str,
    ) -> Self {
        Self {
            q_id,
            q_start,
            q_end,
            q_dir,
            e_id,
            e_len,
            e_sq,
            e_ac,
            e_desc,
        }
    }
}

impl<'a> std::fmt::Display for SearchedDesc<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t\n",
            self.q_id,
            self.q_start,
            self.q_end,
            self.q_dir,
            self.e_id,
            self.e_len,
            self.e_sq,
            self.e_ac,
            self.e_desc
        )
    }
}

/// A wrapper type around `Vec<SearchedDesc>` to implement Display.
#[derive(Debug, Clone)]
pub struct SearchedDescList<'a>(pub Vec<SearchedDesc<'a>>);
impl<'a> From<Vec<SearchedDesc<'a>>> for SearchedDescList<'a> {
    fn from(v: Vec<SearchedDesc<'a>>) -> Self {
        Self(v)
    }
}

impl<'a> std::fmt::Display for SearchedDescList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        let header = format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            "Query ID",
            "Query Start",
            "Query End",
            "Query Direction",
            "Element ID",
            "Element Length",
            "Element Sequence",
            "Element Accession",
            "Element Description"
        );
        res.push_str(&header);
        let mut rows = String::new();
        for desc in &self.0 {
            rows.push_str(&desc.to_string());
        }
        write!(f, "{}{}", header, rows)
    }
}
