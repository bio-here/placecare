//!
//! The module is responsible for initializing the PLACE database.
//!

use bincode::config::Configuration;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::place_desc::PlaceDB;

lazy_static! {

    /// The initialized PLACE database,
    /// as the instance of `place_desc:PlaceDB`.
    pub static ref PLACE_DB: PlaceDB = {
        init_place_db().expect("Failed to initialize PLACE database")
    };

    /// The IUPAC code map to regex pattern.
    pub static ref IUPAC_MAP: HashMap<char, &'static str> = {
        init_iupac_map()
    };
}

/// The function will only run in lazy_static context.
///
/// The `init_place_db` function initializes the PLACE database
/// from the file that serialized into the source while building the `placecare`.
fn init_place_db() -> Result<PlaceDB, String> {
    let place_db: &[u8] = include_bytes!("./db_file/place.db");

    let read_config = bincode::config::standard().with_variable_int_encoding();

    let (bin_db, _) =
        bincode::borrow_decode_from_slice::<PlaceDB, Configuration>(place_db, read_config)
            .map_err(|e| format!("Failed to decode place_db: {}", e))?;

    Ok(bin_db)
}

/// The function will initialize the IUPAC map.
fn init_iupac_map() -> HashMap<char, &'static str> {
    let mut m = HashMap::new();
    m.insert('A', "A");
    m.insert('C', "C");
    m.insert('G', "G");
    m.insert('T', "T");
    m.insert('R', "[AG]");
    m.insert('Y', "[CT]");
    m.insert('M', "[AC]");
    m.insert('K', "[GT]");
    m.insert('S', "[GC]");
    m.insert('W', "[AT]");
    m.insert('B', "[CGT]");
    m.insert('D', "[AGT]");
    m.insert('H', "[ACT]");
    m.insert('V', "[ACG]");
    m.insert('N', "[ACGT]");

    // m.insert('A', "[AN]");
    // m.insert('C', "[CN]");
    // m.insert('G', "[GN]");
    // m.insert('T', "[TN]");
    // m.insert('R', "[AGN]");
    // m.insert('Y', "[CTN]");
    // m.insert('M', "[ACN]");
    // m.insert('K', "[GTN]");
    // m.insert('S', "[GCN]");
    // m.insert('W', "[ATN]");
    // m.insert('B', "[CGTN]");
    // m.insert('D', "[AGTN]");
    // m.insert('H', "[ACTN]");
    // m.insert('V', "[ACGN]");
    // m.insert('N', "[ACGTN]");

    m
}
