use std::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchParams {
    pub original_text: String,
    pub src_lang: String,
    pub dst_langs: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub src_lang: String,
    pub dst_lang: String,
    pub translated_text: String,
}
