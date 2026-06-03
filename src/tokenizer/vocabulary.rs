use std::collections::HashSet;

use crate::tokenizer::word_table::WordTable;

pub(crate) struct Vocabulary(HashSet<String>);
impl From<&WordTable> for Vocabulary {
    fn from(value: &WordTable) -> Self {
        // all unique characters (bytes) in the corpus
        let mut out = HashSet::new();
        value.as_ref().keys().for_each(|word| {
            word.iter().for_each(|s| {
                out.insert(s.to_string());
            });
        });
        Vocabulary(out)
    }
}
impl Vocabulary {
    pub fn add(&mut self, token: &String) {
        self.0.insert(token.to_string());
    }
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl AsRef<HashSet<String>> for Vocabulary {
    fn as_ref(&self) -> &HashSet<String> {
        &self.0
    }
}
