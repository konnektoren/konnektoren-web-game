use crate::model::GameLoader;
use konnektoren_core::game::Game;
use konnektoren_core::prelude::ChallengeConfig;

pub struct SearchChallenges {
    game: Game,
}

impl SearchChallenges {
    pub fn new() -> Self {
        let game = Game::load_game().unwrap();
        Self { game }
    }

    pub fn search(&self, search_term: &str) -> Vec<ChallengeConfig> {
        let mut challenges = vec![];
        for game_path in &self.game.game_paths {
            for challenge in &game_path.challenges {
                if challenge.name.contains(&search_term) {
                    challenges.push(challenge.clone());
                }
            }
        }
        challenges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_challenges() {
        let search_challenges = SearchChallenges::new();
        let challenges = search_challenges.search("Konnektoren");
        assert_eq!(challenges.len(), 7);
    }
}
