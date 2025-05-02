//!
//! This module is responsible for searching the PLACE database.

use crate::db::IUPAC_MAP;
use crate::io::{RecordDesc, SearchResult};
use crate::place_desc::SeqDesc;
use crate::{db::PLACE_DB, io::SearchedDesc};
use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex_automata::Input;
use regex_automata::MatchKind;
use regex_automata::dfa::Automaton;
use regex_automata::dfa::OverlappingState;
use regex_automata::dfa::dense;

use std::sync::Mutex;

lazy_static! {
    /// The initialized Aho-Corasick automaton.
    pub static ref AC: AhoCorasick = build_ac();

}

fn build_ac() -> AhoCorasick {
    AhoCorasick::builder()
        .match_kind(aho_corasick::MatchKind::Standard)
        .ascii_case_insensitive(false)
        .build(PLACE_DB.seq_desc.exact.iter().map(|x| x.sq.as_str()))
        .unwrap()
}

pub struct Search;

impl Search {
    /// The function is to search element in PLACE database,
    /// by input your query string that is fasta format.
    ///
    /// When there's multiple sequences in the query,
    /// The function will run multiple threads to increase the speed.
    ///
    pub fn search_elements(
        query: &[RecordDesc],
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let res_p: Mutex<Vec<SearchResult>> = Mutex::new(Vec::new());

        query
            .par_iter()
            .for_each(|seq| match Self::search_elements_single_seq(&seq) {
                Ok(r) => {
                    res_p.lock().unwrap().extend(r);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            });

        let res = res_p.lock().unwrap().to_owned();
        Ok(res)
    }

    /// The function is to search on 1 sequence.
    pub fn search_elements_single_seq(
        query: &RecordDesc,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut total: Vec<SearchResult> = vec![];
        let mut searched: Vec<SearchedDesc> = vec![];

        let res_exact = Self::search_element_exact(query)?;
        let res_iupac = Self::search_element_iupac(query)?;
        searched.extend(res_exact);
        searched.extend(res_iupac);
        searched.sort_unstable_by(|a, b| a.q_start.cmp(&b.q_start));

        total.push(SearchResult::new(
            &query.id, // id
            searched,  // search results
        ));

        Ok(total)
    }

    /// Search element by exact match with Aho-Corasick algorithm.
    fn search_element_exact(
        query: &RecordDesc,
    ) -> Result<Vec<SearchedDesc>, Box<dyn std::error::Error>> {
        let seqs = &PLACE_DB.seq_desc.exact;
        let mut descs = Vec::with_capacity(200);

        for mat in AC.find_overlapping_iter(&query.seq) {
            let pat = mat.pattern();
            let seq = &seqs[pat];
            let searched = SearchedDesc::new_from_ref(
                &query.id,       // id
                mat.start() + 1, // start position
                mat.end() + 1,   // end position
                1,               // sequence direction
                &seq.id,         // element id
                seq.sq.len(),    // element length
                &seq.sq,         // element sequence
                &seq.ac,         // element accession number
                &seq.de,         // element description
            );
            descs.push(searched);
        }

        let reverse = Self::reverse_complement(&query.seq);
        for mat in AC.find_overlapping_iter(&reverse) {
            let pat = mat.pattern();
            let seq = &seqs[pat];
            let searched = SearchedDesc::new(
                query.id.clone(),            // id
                query.len + 1 - mat.end(),   // start position
                query.len + 1 - mat.start(), // end position
                0,                           // sequence direction
                seq.id.clone(),              // element id
                seq.sq.len(),                // element length
                seq.sq.clone(),              // element sequence
                seq.ac.clone(),              // element accession number
                seq.de.clone(),              // element description
            );
            descs.push(searched);
        }

        Ok(descs)
    }

    /// Search element by IUPAC match with Regex.
    fn search_element_iupac(
        query: &RecordDesc,
    ) -> Result<Vec<SearchedDesc>, Box<dyn std::error::Error>> {
        let seqs = &PLACE_DB.seq_desc.iupac;
        let mut descs = Vec::with_capacity(200);

        let _iupac_regex = seqs
            .iter()
            .map(|x| Self::iupac_2_regex(&x.sq))
            .collect::<Vec<_>>();

        seqs.iter().for_each(|seq| {
            let iupac_regex = Self::iupac_2_regex(&seq.sq);

            let dfa = dense::DFA::builder()
                .configure(dense::DFA::config().match_kind(MatchKind::All))
                .build(&iupac_regex)
                .unwrap();

            let input = Input::new(&query.seq);
            let mut state = OverlappingState::start();
            while let Some(hm) = {
                dfa.try_search_overlapping_fwd(&input, &mut state).unwrap();
                state.get_match()
            } {
                let pat = hm.pattern();
                let seq = &seqs[pat];

                let end = hm.offset();
                let start = end - seq.sq.len();
                let searched = SearchedDesc::new_from_ref(
                    &query.id,    // id
                    start + 1,    // start position
                    end + 1,      // end position
                    1,            // sequence direction
                    &seq.id,      // element id
                    seq.sq.len(), // element length
                    &seq.sq,      // element sequence
                    &seq.ac,      // element accession number
                    &seq.de,      // element description
                );
                descs.push(searched);
            }

            let reverse = Self::reverse_complement(&query.seq);
            let input = Input::new(&reverse);
            let mut state = OverlappingState::start();
            while let Some(hm) = {
                dfa.try_search_overlapping_fwd(&input, &mut state).unwrap();
                state.get_match()
            } {
                let pat = hm.pattern();
                let seq = &seqs[pat];

                let end = hm.offset();
                let start = end - seq.sq.len();
                let searched = SearchedDesc::new_from_ref(
                    &query.id,             // id
                    query.len + 1 - end,   // start position
                    query.len + 1 - start, // end position
                    0,                     // sequence direction
                    &seq.id,               // element id
                    seq.sq.len(),          // element length
                    &seq.sq,               // element sequence
                    &seq.ac,               // element accession number
                    &seq.de,               // element description
                );
                descs.push(searched);
            }
        });

        Ok(descs)
    }

    /// Query elements by ID.
    pub fn query_elements_by_id(query: &[&str]) -> Vec<Option<SeqDesc>> {
        let map = &PLACE_DB.seq_index.id_index;
        let mut elements = Vec::new();

        for &id in query {
            if let Some(&index) = map.get(id) {
                elements.push(Some(PLACE_DB.seq_desc.all[index].clone()));
            } else {
                elements.push(None);
            }
        }
        elements
    }

    /// Query elements by AC.
    pub fn query_elements_by_ac(query: &[&str]) -> Vec<Option<SeqDesc>> {
        println!("{:?}", query);
        let map = &PLACE_DB.seq_index.ac_index;
        let mut elements = Vec::new();

        for &ac in query {
            if let Some(&index) = map.get(ac) {
                elements.push(Some(PLACE_DB.seq_desc.all[index].clone()));
            } else {
                elements.push(None);
            }
        }
        elements
    }
}

/// Tool functions
impl Search {
    /// Convert IUPAC code to regex pattern.
    pub(crate) fn iupac_2_regex(query: &str) -> String {
        let mut regex = String::with_capacity(query.len() * 3);
        for c in query.chars() {
            if let Some(&pattern) = IUPAC_MAP.get(&c) {
                regex.push_str(pattern);
            } else {
                regex.push(c);
            }
        }

        regex
    }

    pub(crate) fn reverse_complement(query: &str) -> String {
        let mut rev_comp = String::with_capacity(query.len());
        for c in query.chars().rev() {
            match c {
                'A' => rev_comp.push('T'),
                'T' => rev_comp.push('A'),
                'C' => rev_comp.push('G'),
                'G' => rev_comp.push('C'),
                _ => rev_comp.push(c),
            }
        }
        rev_comp
    }
}
