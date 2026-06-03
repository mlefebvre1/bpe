mod trainer;
mod vocabulary;
mod word_table;

use std::collections::HashSet;

use crate::tokenizer::{trainer::TokenizerTrainer, vocabulary::Vocabulary};

#[derive(Debug)]
pub struct MergeRule((String, String));

pub struct Tokenizer {
    vocabulary: Vocabulary,
    merge_rules: Vec<MergeRule>,
}

impl Tokenizer {
    pub fn trainer(corpus: &str) -> TokenizerTrainer<'_> {
        TokenizerTrainer::new(corpus)
    }
    pub fn vocabulary(&self) -> &HashSet<String> {
        self.vocabulary.as_ref()
    }
    pub fn merge_rules(&self) -> &[MergeRule] {
        &self.merge_rules
    }
    pub fn encode(&self, text: &str) -> Vec<String> {
        // This pre-process could be shared.
        let words = text.split_whitespace().map(|word| {
            format!("{word}</")
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        });
        let mut result = vec![];
        for mut word in words {
            // This pair merging algorithm could be shared
            for MergeRule((left, right)) in &self.merge_rules {
                let merged = format!("{}{}", left, right);
                let mut new_word = vec![];
                let mut i = 0;
                while i < word.len() {
                    if i + 1 < word.len() && &word[i] == left && &word[i + 1] == right {
                        new_word.push(merged.clone());
                        i += 2;
                    } else {
                        new_word.push(word[i].clone());
                        i += 1;
                    }
                }
                word = new_word;
            }
            for token in word {
                result.push(token);
            }
        }
        result
    }
    pub fn decode(&self, tokens: &[String]) -> String {
        tokens.join("").replace("</", " ")
    }
}
