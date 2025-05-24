//!
//! This module is responsible for searching the PLACE database.

use crate::db::IUPAC_MAP;
use crate::io::{RecordDesc, SearchResult};
use crate::place_desc::SeqDesc;
use crate::{db::PLACE_DB, io::SearchedDesc};
use rayon::prelude::*;

use std::sync::Mutex;

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
        let mut res_p: Vec<SearchResult> = Vec::new();

        // not gonna use rayon as it's a overhead for small data.
        query
            .iter()
            .for_each(|seq| match Self::search_elements_single_seq(&seq) {
                Ok(r) => {
                    res_p.extend(r);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            });

        let res = res_p.to_owned();
        Ok(res)
    }

    /// The function is to search on 1 sequence.
    pub fn search_elements_single_seq(
        query: &RecordDesc,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut searched: Vec<SearchedDesc> = vec![];
        let pre_size = query.len() / 5;

        let res_exact = Self::search_element_exact(query, pre_size)?;
        let res_iupac = Self::search_element_iupac(query, pre_size)?;
        searched.extend(res_exact);
        searched.extend(res_iupac);
        searched.sort_unstable_by(|a, b| a.q_start.cmp(&b.q_start));

        let total = vec![SearchResult::new(
            &query.id(), // id
            searched,    // search results
        )];

        Ok(total)
    }

    /// Search element by exact match with KMP algorithm.
    fn search_element_exact(
        query: &RecordDesc,
        presize: usize,
    ) -> Result<Vec<SearchedDesc>, Box<dyn std::error::Error>> {
        let seqs = &PLACE_DB.seq_desc.exact;
        let descs = Mutex::new(Vec::with_capacity(presize));

        // Search the forward sequence
        seqs.par_iter().for_each(|pattern| {
            let matches = Self::kmp_search(&query.seq(), &pattern.sq);
            let mut partial_descs = Vec::with_capacity(matches.len());
            for &start in &matches {
                let end = start + pattern.sq.len();
                let searched = SearchedDesc::new(
                    &query.id(),      // id
                    start + 1,        // start position (1-based)
                    end + 1,          // end position (1-based)
                    1,                // sequence direction
                    &pattern.id,      // element id
                    pattern.sq.len(), // element length
                    &pattern.sq,      // element sequence
                    &pattern.ac,      // element accession number
                    &pattern.de,      // element description
                );
                partial_descs.push(searched);
            }
            descs.lock().unwrap().extend(partial_descs);
        });

        // Search the reverse complement sequence
        let reverse = Self::reverse_complement(&query.seq());
        seqs.par_iter().for_each(|pattern| {
            let matches = Self::kmp_search(&reverse, &pattern.sq);
            let mut partial_descs = Vec::with_capacity(matches.len());
            for &start in &matches {
                let end = start + pattern.sq.len();
                let searched = SearchedDesc::new(
                    &query.id(),             // id
                    query.len() + 1 - end,   // start position
                    query.len() + 1 - start, // end position
                    0,                       // sequence direction
                    &pattern.id,             // element id
                    pattern.sq.len(),        // element length
                    &pattern.sq,             // element sequence
                    &pattern.ac,             // element accession number
                    &pattern.de,             // element description
                );
                partial_descs.push(searched);
            }
            descs.lock().unwrap().extend(partial_descs);
        });

        Ok(descs.into_inner().unwrap())
    }

    /// Search element by IUPAC match with KMP-based pattern matching.
    fn search_element_iupac(
        query: &RecordDesc,
        presize: usize,
    ) -> Result<Vec<SearchedDesc>, Box<dyn std::error::Error>> {
        let seqs = &PLACE_DB.seq_desc.iupac;
        let descs = Mutex::new(Vec::with_capacity(presize));

        seqs.par_iter().for_each(|pattern| {
            // Search the forward sequence
            let matches = Self::kmp_search_with_iupac(&query.seq(), &pattern.sq);
            let mut partial_descs = Vec::with_capacity(matches.len());
            for &start in &matches {
                let end = start + pattern.sq.len();
                let searched = SearchedDesc::new(
                    &query.id(),  // id
                    start + 1,    // start position (1-based)
                    end + 1,      // end position (1-based)
                    1,            // sequence direction
                    &pattern.id,      // element id
                    pattern.sq.len(), // element length
                    &pattern.sq,      // element sequence
                    &pattern.ac,      // element accession number
                    &pattern.de,      // element description
                );
                partial_descs.push(searched);
            }
            descs.lock().unwrap().extend(partial_descs);
        });

        seqs.par_iter().for_each(|pattern| {
            // Search the reverse complement sequence
            let reverse = Self::reverse_complement(&query.seq());
            let matches = Self::kmp_search_with_iupac(&reverse, &pattern.sq);
            let mut partial_descs = Vec::with_capacity(matches.len());
            for &start in &matches {
                let end = start + pattern.sq.len();
                let searched = SearchedDesc::new(
                    &query.id(),             // id
                    query.len() + 1 - end,   // start position
                    query.len() + 1 - start, // end position
                    0,                       // sequence direction
                    &pattern.id,                 // element id
                    pattern.sq.len(),            // element length
                    &pattern.sq,                 // element sequence
                    &pattern.ac,                 // element accession number
                    &pattern.de,                 // element description
                );
                partial_descs.push(searched);
            }
            descs.lock().unwrap().extend(partial_descs);
        });

        Ok(descs.into_inner().unwrap())
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

/// KMP Algorithm
impl Search {
    /// Compute the longest prefix-suffix (LPS) array
    fn compute_lps(pattern: &str) -> Vec<usize> {
        let chars: Vec<char> = pattern.chars().collect();
        let n = chars.len();
        let mut lps = vec![0; n];

        let mut len = 0;
        let mut i = 1;

        while i < n {
            if chars[i] == chars[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }

        lps
    }

    /// KMP search algorithm
    fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
        let mut matches = Vec::new();
        if pattern.is_empty() {
            return matches;
        }

        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        let n = text_chars.len();
        let m = pattern_chars.len();

        if m > n {
            return matches;
        }

        let lps = Self::compute_lps(pattern);

        let mut i = 0; // Text pointer
        let mut j = 0; // Pattern pointer

        while i < n {
            if pattern_chars[j] == text_chars[i] {
                i += 1;
                j += 1;
            }

            if j == m {
                matches.push(i - j);
                j = lps[j - 1];
            } else if i < n && pattern_chars[j] != text_chars[i] {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }

        matches
    }

    /// KMP search algorithm with IUPAC support
    fn kmp_search_with_iupac(text: &str, pattern: &str) -> Vec<usize> {
        let mut matches = Vec::new();
        if pattern.is_empty() {
            return matches;
        }

        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        let n = text_chars.len();
        let m = pattern_chars.len();

        if m > n {
            return matches;
        }

        // Check if the pattern matches
        'outer: for i in 0..=n - m {
            for j in 0..m {
                if !Self::is_iupac_match(text_chars[i + j], pattern_chars[j]) {
                    continue 'outer;
                }
            }
            matches.push(i);
        }

        matches
    }
}

/// Tool functions
impl Search {
    /// Check if a character in the text matches the IUPAC pattern
    fn is_iupac_match(text_char: char, pattern_char: char) -> bool {
        if pattern_char == text_char {
            return true;
        }

        if let Some(pattern) = IUPAC_MAP.get(&pattern_char) {
            let regex_pattern = pattern.to_string();
            if regex_pattern.len() == 1 {
                return regex_pattern.chars().next().unwrap() == text_char;
            } else {
                let mut chars_inside_brackets = regex_pattern
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .chars();
                return chars_inside_brackets.any(|c| c == text_char);
            }
        }

        false
    }

    /// Get reverse complement chain
    fn reverse_complement(query: &str) -> String {
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
