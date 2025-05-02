//!
//! This module contains a structure of the PLACE database.

use bincode::Decode;
use comfy_table::Table;
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
        let header = vec![
            "ID", "AC", "DT", "DE", "KW", "OS", "RA", "RT", "RL", "RD", "RC", "SQ",
        ];

        let row = vec![
            self.id.clone(),
            self.ac.clone(),
            self.dt.clone(),
            self.de.clone(),
            self.kw.join(", "),
            self.os.clone(),
            self.ra.clone(),
            self.rt.clone(),
            self.rl.clone(),
            self.rd.clone(),
            self.rc.clone(),
            self.sq.clone(),
        ];

        let mut table = Table::new();
        table
            .set_header(header)
            .add_row(row)
            .load_preset(comfy_table::presets::NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
        write!(f, "{}", table)
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
