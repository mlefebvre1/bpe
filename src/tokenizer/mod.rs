mod merge_rule;
mod trainer;
mod vocabulary;
mod word_table;

use crate::tokenizer::{merge_rule::MergeRule, trainer::TokenizerTrainer, vocabulary::Vocabulary};

pub struct Tokenizer {
    vocabulary: Vocabulary,
    merge_rules: Vec<MergeRule>,
}

impl Tokenizer {
    pub fn trainer(corpus: &str) -> TokenizerTrainer<'_> {
        TokenizerTrainer::new(corpus)
    }
    pub fn encode(&self, text: &str) -> Vec<usize> {
        let words = Self::pre_process(text);
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
                match self.vocabulary.token_from_word(&token) {
                    Some(token_index) => {
                        result.push(token_index);
                    }
                    None => {
                        println!("token '{token}' not found in the vocabulary! skipping it");
                    }
                }
            }
        }
        result
    }
    pub fn decode(&self, tokens: &[usize]) -> String {
        tokens
            .iter()
            .map(|token_index| {
                self.vocabulary
                    .word_from_token(*token_index)
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("")
            .replace("</", " ")
    }

    fn pre_process(text: &str) -> impl Iterator<Item = Vec<String>> {
        text.split_whitespace()
            .map(|word| format!("{word}</").chars().map(|c| c.to_string()).collect())
    }
}
