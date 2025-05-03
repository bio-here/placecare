use bincode::Encode;
use env_logger::Builder;
use log::info;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::File;

///
/// This file will be run before the main build process.
/// Some jobs are done here:
///
/// 1. Log the building process.
/// 2. Serialize the place.seq files.
///

fn main() {
    let _out_dir = std::env::var("OUT_DIR").unwrap();
    Builder::new().filter_level(log::LevelFilter::Info).init();

    if std::path::Path::new("src/db_file/place.db").exists() {
        info!("place.db already exists, skipping serialization.");
        return;
    }
    place_db_serialize();
}

fn place_db_serialize() {
    let place_seq_file =
        fs::read_to_string("./db_place/place.seq").expect("File<place.seq> not found");

    let seq_descs = SeqDesc::from_place_seq(&place_seq_file);
    let place_index = PlaceIndex::from_descs(&seq_descs);
    let place_seq = SeqBuilder::from_seq_builder(seq_descs);
    let place_db = PlaceDB::new(place_seq, place_index);

    let seq_db = File::create("src/db_file/place.db").expect("Failed to open file for writing");
    let config = bincode::config::standard().with_variable_int_encoding();

    let mut writer = std::io::BufWriter::new(seq_db);
    let _length = bincode::encode_into_std_write(place_db, &mut writer, config)
        .expect("Failed to serialize place_db");
}

#[derive(Encode, Debug)]
pub struct PlaceDB {
    pub seq_desc: SeqBuilder,
    pub seq_index: PlaceIndex,
}

impl PlaceDB {
    fn new(desc: SeqBuilder, index: PlaceIndex) -> PlaceDB {
        PlaceDB {
            seq_desc: desc,
            seq_index: index,
        }
    }
}
#[derive(Encode, Debug)]
pub struct SeqBuilder {
    pub exact: Vec<SeqDesc>,
    pub iupac: Vec<SeqDesc>,
    pub all: Vec<SeqDesc>,
}

impl SeqBuilder {
    pub fn from_seq_builder(seq: Vec<SeqDesc>) -> SeqBuilder {
        let (mut v_exact, mut v_iupac) = (Vec::new(), Vec::new());

        seq.clone()
            .into_iter()
            .filter(|desc| desc.sq.chars().all(|c| matches!(c, 'A' | 'C' | 'G' | 'T')))
            .for_each(|desc| {
                v_exact.push(desc);
            });

        seq.clone()
            .into_iter()
            .filter(|desc| {
                desc.sq.chars().any(|c| {
                    matches!(
                        c,
                        'R' | 'Y' | 'M' | 'K' | 'S' | 'W' | 'B' | 'D' | 'H' | 'V' | 'N'
                    )
                })
            })
            .for_each(|desc| {
                v_iupac.push(desc);
            });

        SeqBuilder {
            exact: v_exact,
            iupac: v_iupac,
            all: seq,
        }
    }
}

#[derive(Encode, Debug, Clone)]
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

impl SeqDesc {
    fn new(
        id: String,
        ac: String,
        dt: String,
        de: String,
        kw: Vec<String>,
        os: String,
        ra: String,
        rt: String,
        rl: String,
        rd: String,
        rc: String,
        sq: String,
    ) -> Self {
        Self {
            id,
            ac,
            dt,
            de,
            kw,
            os,
            ra,
            rt,
            rl,
            rd,
            rc,
            sq,
        }
    }

    fn from_place_seq(seq: &str) -> Vec<SeqDesc> {
        // Length of seq is 470, based on place.seq
        let mut descs: Vec<SeqDesc> = Vec::with_capacity(470);

        seq.split("//")
            .filter(|s| !s.trim().is_empty())
            .for_each(|seq_block| {
                let lines: Vec<_> = seq_block
                    .lines()
                    .map(|l| l.trim())
                    .filter(|line| !line.is_empty())
                    .filter(|line| !line.starts_with("XX"))
                    .collect();

                let (&id, rest) = lines.split_first().unwrap();
                let (&sq, rest) = rest.split_last().unwrap();
                let id = id[2..].trim().to_string();
                let sq = sq.trim().to_string();

                let mut ac = String::new();
                let mut dt = String::new();
                let mut de = String::new();
                let mut kw = Vec::new();
                let mut os = String::new();
                let mut ra = String::new();
                let mut rt = String::new();
                let mut rl = String::new();
                let mut rd = String::new();
                let mut rc = String::new();

                for &line in rest {
                    if line.len() < 2 {
                        continue;
                    }

                    match &line[0..2] {
                        "ID" | "SQ" => {} // Processed on line 63, 64
                        "AC" => ac.push_str(&line[2..].trim()),
                        "DT" => dt.push_str(&line[2..].trim()),
                        "DE" => de.push_str(&line[2..].trim()),
                        "KW" => {
                            kw.extend(
                                line[2..]
                                    .trim()
                                    .split(';')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty()),
                            );
                        }
                        "OS" => os.push_str(&line[2..].trim()),
                        "RA" => ra.push_str(&line[2..].trim()),
                        "RT" => rt.push_str(&line[2..].trim()),
                        "RL" => rl.push_str(&line[2..].trim()),
                        "RD" => rd.push_str(&line[2..].trim()),
                        "RC" => rc.push_str(&line[2..].trim()),
                        "" => {} // Other empty lines
                        _ => {
                            info!("Unknown line: {:?}", line);
                        }
                    }
                }

                descs.push(SeqDesc::new(id, ac, dt, de, kw, os, ra, rt, rl, rd, rc, sq));
            });
        // let mut writer =
        //     std::io::BufWriter::new(File::create("build.logs").expect("Failed to open log file"));
        // for desc in &descs {
        //     writeln!(&mut writer, "{:?}", desc).expect("Failed to write to log file");
        // }
        descs
    }
}

impl fmt::Display for SeqDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}, AC: {}, DT: {}, DE: {}, KW: {:?}, OS: {}, RA: {}, RT: {}, RL: {}, RD: {}, SQ: {}",
            self.id,
            self.ac,
            self.dt,
            self.de,
            self.kw,
            self.os,
            self.ra,
            self.rt,
            self.rl,
            self.rd,
            self.sq
        )
    }
}

#[derive(Encode, Debug)]
pub struct PlaceIndex {
    pub id_index: HashMap<String, usize>,
    pub ac_index: HashMap<String, usize>,
    // pub sq_index: HashMap<String, usize>   // Not unique
}

impl PlaceIndex {
    pub fn from_descs(descs: &[SeqDesc]) -> PlaceIndex {
        let mut id_index = HashMap::new();
        let mut ac_index = HashMap::new();

        for (p, desc) in descs.iter().enumerate() {
            id_index.insert(desc.id.clone(), p);
            ac_index.insert(desc.ac.clone(), p);
        }

        Self { id_index, ac_index }
    }
}
