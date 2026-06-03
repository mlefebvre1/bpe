use std::collections::HashMap;

pub(crate) struct WordTable(HashMap<Vec<String>, u64>);
impl WordTable {
    pub fn from_corpus(corpus: &str) -> Self {
        let words = corpus.split_whitespace();

        // Add a special end-of-word marker to each word. ("</")
        let words = words.map(|word| {
            format!("{word}</")
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        });

        let frequency_table = Self::frequency_table(words);
        Self(frequency_table)
    }
    fn frequency_table(words: impl Iterator<Item = Vec<String>>) -> HashMap<Vec<String>, u64> {
        let mut out = HashMap::new();
        words.for_each(|word| match out.get_mut(&word.clone()) {
            Some(freq) => {
                *freq += 1;
            }
            None => {
                out.insert(word, 0u64);
            }
        });
        out
    }

    /// Update the word table with the new merged pair
    pub fn update(&mut self, merged_pair: &(String, String)) {
        let mut new_word_table = HashMap::new();

        let (target_left, target_right) = merged_pair;
        let merged = format!("{}{}", target_left, target_right);

        for (word, count) in self.0.drain() {
            let mut new_word = vec![];
            let mut i = 0;
            while i < word.len() {
                match (word.get(i), word.get(i + 1)) {
                    (Some(left), Some(right)) => {
                        if left == target_left && right == target_right {
                            new_word.push(merged.clone());
                            i += 2;
                        } else if i == word.len() - 1 {
                            new_word.push(left.to_string());
                            new_word.push(right.to_string());
                            i += 1;
                        } else {
                            new_word.push(left.to_string());
                            i += 1;
                        }
                    }
                    (Some(left), None) => {
                        new_word.push(left.to_string());
                        i += 1;
                    }
                    _ => {
                        // cannot occur
                        break;
                    }
                }
            }

            new_word_table.insert(new_word, count);
        }

        self.0 = new_word_table;
    }
}

impl AsRef<HashMap<Vec<String>, u64>> for WordTable {
    fn as_ref(&self) -> &HashMap<Vec<String>, u64> {
        &self.0
    }
}
