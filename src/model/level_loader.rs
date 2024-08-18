use crate::model::WebSession;
use konnektoren_core::game::{Game, GamePath, GameState};
use konnektoren_core::prelude::{Challenge, ChallengeFactory, ChallengeType, Session};

pub trait LevelLoader<T> {
    fn level_a1() -> T;

    fn level_a2() -> T;

    fn level_c1() -> T;
}

impl LevelLoader<ChallengeFactory> for ChallengeFactory {
    fn level_a1() -> ChallengeFactory {
        let data = include_str!("../assets/challenges/articles.yml");
        let articles: ChallengeType = serde_yaml::from_str(data).unwrap();

        let articles_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/articles_info.yml")).unwrap();

        let data = include_str!("../assets/challenges/reflexivpronouns.yml");
        let reflexivpronouns: ChallengeType = serde_yaml::from_str(data).unwrap();

        let reflexivpronouns_info: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/reflexivpronouns_info.yml"
        ))
        .unwrap();

        let data = include_str!("../assets/challenges/personal_pronouns.yml");
        let personalpronouns: ChallengeType = serde_yaml::from_str(data).unwrap();

        let personalpronouns_info: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/personal_pronouns_info.yml"
        ))
        .unwrap();

        let data = include_str!("../assets/challenges/konnektoren.yml");
        let connectives: ChallengeType = serde_yaml::from_str(data).unwrap();

        let connectives_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/connectives_info.yml"))
                .unwrap();

        ChallengeFactory {
            challenge_types: vec![
                articles,
                articles_info,
                reflexivpronouns,
                reflexivpronouns_info,
                personalpronouns,
                personalpronouns_info,
                connectives,
                connectives_info,
            ],
        }
    }

    fn level_a2() -> ChallengeFactory {
        let data = include_str!("../assets/challenges/articles.yml");
        let articles: ChallengeType = serde_yaml::from_str(data).unwrap();

        let articles_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/articles_info.yml")).unwrap();

        ChallengeFactory {
            challenge_types: vec![articles, articles_info],
        }
    }

    fn level_c1() -> ChallengeFactory {
        let data = include_str!("../assets/challenges/konnektoren.yml");
        let connectives: ChallengeType = serde_yaml::from_str(data).unwrap();

        let connectives_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/connectives_info.yml"))
                .unwrap();

        ChallengeFactory {
            challenge_types: vec![connectives, connectives_info],
        }
    }
}

impl LevelLoader<GamePath> for GamePath {
    fn level_a1() -> GamePath {
        let articles: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_a1.yml")).unwrap();
        articles
    }

    fn level_a2() -> GamePath {
        let articles: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_a2.yml")).unwrap();
        articles
    }

    fn level_c1() -> GamePath {
        let articles: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_c1.yml")).unwrap();
        articles
    }
}

impl LevelLoader<GameState> for GameState {
    fn level_a1() -> GameState {
        GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::level_a1(),
            challenge: Challenge::default(),
            current_task_index: 0,
        }
    }

    fn level_a2() -> GameState {
        GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::level_a2(),
            challenge: Challenge::default(),
            current_task_index: 0,
        }
    }

    fn level_c1() -> GameState {
        GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::level_c1(),
            challenge: Challenge::default(),
            current_task_index: 0,
        }
    }
}

impl LevelLoader<Game> for Game {
    fn level_a1() -> Game {
        let game_paths = vec![GamePath::level_a1()];
        let challenge_factory = ChallengeFactory::level_a1();
        Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        }
    }

    fn level_a2() -> Game {
        let game_paths = vec![GamePath::level_a2()];
        let challenge_factory = ChallengeFactory::level_a2();
        Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        }
    }

    fn level_c1() -> Game {
        let game_paths = vec![GamePath::level_c1()];
        let challenge_factory = ChallengeFactory::level_c1();
        Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        }
    }
}

impl LevelLoader<Session> for Session {
    fn level_a1() -> Session {
        Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_a1(),
        }
    }

    fn level_a2() -> Session {
        Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_a2(),
        }
    }

    fn level_c1() -> Session {
        Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_c1(),
        }
    }
}

impl LevelLoader<WebSession> for WebSession {
    fn level_a1() -> WebSession {
        WebSession {
            id: "websession".into(),
            session: Session::level_a1(),
        }
    }

    fn level_a2() -> WebSession {
        WebSession {
            id: "websession".into(),
            session: Session::level_a2(),
        }
    }

    fn level_c1() -> WebSession {
        WebSession {
            id: "websession".into(),
            session: Session::level_c1(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_a1() {
        let game = Game::level_a1();
        assert_eq!(game.game_paths[0].challenges.len(), 12);
    }

    #[test]
    fn test_level_c1() {
        let game = Game::level_c1();
        assert_eq!(game.game_paths[0].challenges.len(), 5);
    }

    #[test]
    fn test_level_a1_session() {
        let session = WebSession::level_a1();
        assert_eq!(
            session.session.game_state.game.game_paths[0]
                .challenges
                .len(),
            12
        );
    }

    #[test]
    fn test_level_c1_session() {
        let session = WebSession::level_c1();
        assert_eq!(
            session.session.game_state.game.game_paths[0]
                .challenges
                .len(),
            5
        );
    }
}
