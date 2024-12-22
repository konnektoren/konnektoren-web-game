use crate::model::GameLoader;
use konnektoren_core::game::Game;
use konnektoren_core::prelude::ChallengeConfig;
use std::collections::{HashMap, HashSet};
use strsim::normalized_levenshtein;

pub struct SearchChallenges {
    challenge_index: HashMap<String, Vec<(ChallengeConfig, f64)>>,
}

impl SearchChallenges {
    pub fn new() -> Self {
        let game = Game::load_game().unwrap();
        let mut challenge_index = HashMap::new();

        for game_path in &game.game_paths {
            for challenge in &game_path.challenges {
                let name_words = Self::tokenize(&challenge.name);
                let desc_words = Self::tokenize(&challenge.description);

                let all_words: HashSet<_> = name_words.into_iter().chain(desc_words).collect();

                for word in all_words {
                    challenge_index
                        .entry(word)
                        .or_insert_with(Vec::new)
                        .push((challenge.clone(), 0.0));
                }
            }
        }

        Self { challenge_index }
    }

    pub fn search(&self, search_term: &str) -> Vec<ChallengeConfig> {
        let search_words = Self::tokenize(search_term);
        let mut results = HashMap::new();

        for word in &search_words {
            for (index_word, challenges) in &self.challenge_index {
                let similarity = Self::calculate_similarity(word, index_word);
                if similarity > 0.7 {
                    // Adjust this threshold as needed
                    for (challenge, _) in challenges {
                        results
                            .entry(challenge.id.clone())
                            .or_insert_with(|| (challenge.clone(), 0.0))
                            .1 += similarity;
                    }
                }
            }
        }

        let mut sorted_results: Vec<_> = results.into_values().collect();
        sorted_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        sorted_results
            .into_iter()
            .map(|(challenge, _)| challenge)
            .collect()
    }

    pub fn get_suggestion(&self, partial_word: &str) -> Option<String> {
        if partial_word.is_empty() {
            return None;
        }

        let partial_word = partial_word.to_lowercase();
        self.challenge_index
            .keys()
            .filter(|word| {
                word.to_lowercase().contains(&partial_word)
                    || normalized_levenshtein(&partial_word, &word.to_lowercase()) > 0.7
            })
            .max_by(|a, b| {
                let a_sim = normalized_levenshtein(&partial_word, &a.to_lowercase());
                let b_sim = normalized_levenshtein(&partial_word, &b.to_lowercase());
                a_sim.partial_cmp(&b_sim).unwrap()
            })
            .cloned()
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|word| word.to_lowercase())
            .filter(|word| !word.is_empty()) // Remove empty strings, but keep short words
            .collect()
    }

    fn calculate_similarity(word1: &str, word2: &str) -> f64 {
        if word1 == word2 {
            return 1.0;
        }
        normalized_levenshtein(word1, word2)
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_challenges() {
        let search_challenges = SearchChallenges::new();

        // Test search in name
        let challenges = search_challenges.search("Konnektoren");
        assert!(!challenges.is_empty());

        // Test case-insensitive search
        let challenges_lower = search_challenges.search("konnektoren");
        assert_eq!(challenges.len(), challenges_lower.len());

        // Test similar word search
        let challenges_similar = search_challenges.search("Konektoren");
        assert!(!challenges_similar.is_empty());
    }
}
