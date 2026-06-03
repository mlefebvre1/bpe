use std::collections::{HashMap, HashSet};

use crate::tokenizer::{MergeRule, Tokenizer, vocabulary::Vocabulary, word_table::WordTable};

pub struct TokenizerTrainer<'a> {
    corpus: &'a str,
}
impl<'a> TokenizerTrainer<'a> {
    pub(crate) fn new(corpus: &'a str) -> Self {
        Self { corpus }
    }

    fn make_pairs(word_table: &WordTable) -> HashSet<(&String, &String)> {
        let mut pairs = HashSet::new();
        for word in word_table.as_ref().keys() {
            for pair in word.windows(2) {
                pairs.insert((pair.first().unwrap(), pair.get(1).unwrap()));
            }
        }
        pairs
    }
    fn count_adjacent_pairs<'b>(
        word_table: &'b WordTable,
        pairs: &'b HashSet<(&'b String, &'b String)>,
    ) -> HashMap<(&'b String, &'b String), u64> {
        // for each word..
        //      for each pair in the word..
        //          for each pairs to check
        //              if the word pair == the pair to check
        //                  we add the word count of the current word for that pair
        // let mut adjacent_pair_count = HashMap::new();
        let mut adjacent_pair_count: HashMap<(&String, &String), u64> =
            pairs.iter().map(|pair| (*pair, 0u64)).collect();
        for (word, count) in word_table.as_ref().iter() {
            for word_pair in word.windows(2) {
                let (cur_left, cur_right) = (word_pair.first().unwrap(), word_pair.get(1).unwrap());
                for (target_left, target_right) in pairs {
                    if (target_left.as_str() == cur_left.as_str())
                        && (target_right.as_str() == cur_right.as_str())
                    {
                        match adjacent_pair_count.get_mut(&(cur_left, cur_right)) {
                            Some(c) => {
                                *c += *count;
                            }
                            None => {
                                adjacent_pair_count.insert((cur_left, cur_right), *count);
                            }
                        }
                        break;
                    }
                }
            }
        }
        adjacent_pair_count
    }

    pub fn train(self, vocab_size: u64) -> Tokenizer {
        // pre-tokenizer
        let mut word_table = WordTable::from_corpus(self.corpus);
        let mut vocabulary = Vocabulary::from(&word_table);
        let mut merge_rules = vec![];

        // train loop
        loop {
            // find the most frequent pair
            let most_frequent_pair = {
                // make pairs
                let pairs = Self::make_pairs(&word_table);

                // count adjacent pairs across all words, weighted by word frequency
                let adjacent_count = Self::count_adjacent_pairs(&word_table, &pairs);

                let (most_frequent_pair, _) = adjacent_count
                    .iter()
                    .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
                    .unwrap();
                (
                    most_frequent_pair.0.to_string(),
                    most_frequent_pair.1.to_string(),
                )
            };

            // merge the pair
            let new_token = format!("{}{}", most_frequent_pair.0, most_frequent_pair.1);

            // update the word table
            word_table.update(&most_frequent_pair);

            // add the new token to the vocabulary
            vocabulary.add(&new_token);

            // add to merge rule
            merge_rules.push(MergeRule((most_frequent_pair.0, most_frequent_pair.1)));

            if vocabulary.size() >= vocab_size as usize {
                break;
            }
        }
        Tokenizer {
            vocabulary,
            merge_rules,
        }
    }
}
