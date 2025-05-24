//!
//! This module contains a structure of the PLACE database.

use bincode::Decode;
use std::collections::HashMap;

#[derive(Decode, Debug, Clone)]
pub struct PlaceDB {
    pub seq_desc: SeqBuilder,
    pub seq_index: PlaceIndex,
}

impl PlaceDB {
    pub fn new(desc: SeqBuilder, index: PlaceIndex) -> PlaceDB {
        PlaceDB {
            seq_desc: desc,
            seq_index: index,
        }
    }
}

/// The structure split into 2 parts.
#[derive(Decode, Debug, Clone)]
pub struct SeqBuilder {
    pub exact: Vec<SeqDesc>,
    pub iupac: Vec<SeqDesc>,
    pub all: Vec<SeqDesc>,
}

/// This struct is structured according to [place.seq]
///
#[derive(Decode, Debug, Clone)]
pub struct SeqDesc {
    pub id: String,
    pub ac: String,      // accession number
    pub dt: String,      // date
    pub de: String,      // description
    pub kw: Vec<String>, // keywords
    pub os: String,      // organism source
    pub ra: String,      // reference authors
    pub rt: String,      // reference title
    pub rl: String,      // reference location
    pub rd: String,      // reference details
    pub rc: String,      // reference comments
    pub sq: String,      // sequence
}

impl std::fmt::Display for SeqDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let header = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            "ID", "Accession", "Date", "Description", "Keywords", "Organism Source",
            "Reference Authors", "Reference Title", "Reference Location",
            "Reference Details", "Reference Comments", "Sequence");

            let row = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.id, self.ac, self.dt, self.de,
            self.kw.join(", "), self.os, self.ra, self.rt, self.rl,
            self.rd, self.rc, self.sq);

        write!(f, "{}{}", header, row)
    }
}

/// The index of the PLACE database.
/// Make it faster to fetch info
/// by those 2 keys.
#[derive(Decode, Debug, Clone)]
pub struct PlaceIndex {
    pub id_index: HashMap<String, usize>,
    pub ac_index: HashMap<String, usize>,
    // pub sq_index: HashMap<String, usize>  // Not unique
}
