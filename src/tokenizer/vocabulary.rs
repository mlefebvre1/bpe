use indexmap::IndexSet;

use crate::tokenizer::word_table::WordTable;

pub(crate) struct Vocabulary(IndexSet<String>);
impl From<&WordTable> for Vocabulary {
    fn from(value: &WordTable) -> Self {
        // all unique characters (bytes) in the corpus
        let mut out = IndexSet::new();
        value.as_ref().keys().for_each(|word| {
            word.iter().for_each(|s| {
                out.insert(s.to_string());
            });
        });
        Vocabulary(out)
    }
}
impl Vocabulary {
    pub fn add(&mut self, word: &String) {
        self.0.insert(word.to_string());
    }
    pub fn size(&self) -> usize {
        self.0.len()
    }
    pub fn token_from_word(&self, word: &str) -> Option<usize> {
        self.0.iter().position(|cur_word| cur_word == word)
    }
    pub fn word_from_token(&self, token: usize) -> Option<&String> {
        self.0.get_index(token)
    }
}

impl AsRef<IndexSet<String>> for Vocabulary {
    fn as_ref(&self) -> &IndexSet<String> {
        &self.0
    }
}
