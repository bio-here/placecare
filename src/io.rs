//!
//! The IO Module of the DB.

use bio::io::fasta::Records;
use comfy_table::presets::NOTHING;
use std::fmt::Debug;

/// The structure is used to describe the input query sequence.
#[derive(Debug, Clone)]
pub struct RecordDesc {
    pub id: String,  // input query sequence id
    pub seq: String, // input query sequence
    pub len: usize,  // input query sequence length
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
pub struct SearchResult {
    pub id: String,
    pub count: usize,
    pub search_descs: Vec<SearchedDesc>,
}

impl SearchResult {
    pub fn new(id: &str, searched_descs: Vec<SearchedDesc>) -> Self {
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

/// The structure is used to describe the searched element in the database.
#[derive(Clone, Debug)]
pub struct SearchedDesc {
    pub q_id: String,   // input query sequence id
    pub q_start: usize, // start position of the query
    pub q_end: usize,   // end position of the query
    pub q_dir: usize,   // direction of the sequence, 0 for - and 1 for +
    pub e_id: String,   // element id
    pub e_len: usize,   // element length
    pub e_sq: String,   // element sequence
    pub e_ac: String,   // element accession number
    pub e_desc: String, // element description
}

impl SearchedDesc {
    pub fn new(
        q_id: String,
        q_start: usize,
        q_end: usize,
        q_dir: usize,
        e_id: String,
        e_len: usize,
        e_sq: String,
        e_ac: String,
        e_desc: String,
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

    pub fn new_from_ref(
        q_id: &str,
        q_start: usize,
        q_end: usize,
        q_dir: usize,
        e_id: &str,
        e_len: usize,
        e_sq: &str,
        e_ac: &str,
        e_desc: &str,
    ) -> Self {
        Self {
            q_id: q_id.to_owned(),
            q_start,
            q_end,
            q_dir,
            e_id: e_id.to_owned(),
            e_len,
            e_sq: e_sq.to_owned(),
            e_ac: e_ac.to_owned(),
            e_desc: e_desc.to_owned(),
        }
    }
}

impl std::fmt::Display for SearchedDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = vec![
            self.q_id.clone(),
            self.q_start.to_string(),
            self.q_end.to_string(),
            self.q_dir.to_string(),
            self.e_id.clone(),
            self.e_len.to_string(),
            self.e_sq.clone(),
            self.e_ac.clone(),
            self.e_desc.clone(),
        ];
        let mut table = comfy_table::Table::new();
        table
            .load_preset(NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::Disabled);
        table.add_row(row);

        write!(f, "{}", table.to_string())
    }
}

/// A wrapper type around `Vec<SearchedDesc>` to implement Display.
#[derive(Debug, Clone)]
pub struct SearchedDescList(pub Vec<SearchedDesc>);
impl From<Vec<SearchedDesc>> for SearchedDescList {
    fn from(v: Vec<SearchedDesc>) -> Self {
        Self(v)
    }
}

impl std::fmt::Display for SearchedDescList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = vec![
            "Query ID",
            "Query Start",
            "Query End",
            "Query Direction",
            "Element ID",
            "Element Length",
            "Element Sequence",
            "Element Accession Number",
            "Element Description",
        ];
        let mut row = vec![];
        for desc in &self.0 {
            row.push(vec![
                desc.q_id.clone(),
                desc.q_start.to_string(),
                desc.q_end.to_string(),
                desc.q_dir.to_string(),
                desc.e_id.clone(),
                desc.e_len.to_string(),
                desc.e_sq.clone(),
                desc.e_ac.clone(),
                desc.e_desc.clone(),
            ]);
        }
        let mut table = comfy_table::Table::new();
        table
            .set_header(header)
            .load_preset(NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::Disabled);
        for r in row {
            table.add_row(r);
        }
        write!(f, "{}", table.to_string())
    }
}
