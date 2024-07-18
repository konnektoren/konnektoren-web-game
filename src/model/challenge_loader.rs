use crate::model::WebSession;
use konnektoren_core::game::{Game, GamePath, GameState};
use konnektoren_core::prelude::{Challenge, ChallengeFactory, ChallengeType, Session};

pub trait ChallengeLoader<T> {
    fn default_articles() -> T;
    fn default_connectors() -> T;
}

impl ChallengeLoader<ChallengeFactory> for ChallengeFactory {
    fn default_articles() -> ChallengeFactory {
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
        ChallengeFactory {
            challenge_types: vec![
                articles,
                articles_info,
                reflexivpronouns,
                reflexivpronouns_info,
                personalpronouns,
            ],
        }
    }

    fn default_connectors() -> ChallengeFactory {
        Self::default()
    }
}

impl ChallengeLoader<GamePath> for GamePath {
    fn default_articles() -> GamePath {
        let articles: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/articles_path.yml")).unwrap();
        articles
    }

    fn default_connectors() -> GamePath {
        Self::default()
    }
}

impl ChallengeLoader<GameState> for GameState {
    fn default_articles() -> GameState {
        GameState {
            current_challenge_index: 0,
            game: Game::default_articles(),
            challenge: Challenge::default(),
            current_task_index: 0,
        }
    }

    fn default_connectors() -> GameState {
        Self::default()
    }
}

impl ChallengeLoader<Game> for Game {
    fn default_articles() -> Game {
        let game_path = GamePath::default_articles();
        let challenge_factory = ChallengeFactory::default_articles();
        Game {
            game_path,
            challenge_factory,
            challenge_history: Default::default(),
        }
    }

    fn default_connectors() -> Game {
        Self::default()
    }
}

impl ChallengeLoader<Session> for Session {
    fn default_articles() -> Session {
        Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::default_articles(),
        }
    }

    fn default_connectors() -> Session {
        Self::default()
    }
}

impl ChallengeLoader<WebSession> for WebSession {
    fn default_articles() -> WebSession {
        WebSession {
            id: "websession".into(),
            session: Session::default_articles(),
        }
    }

    fn default_connectors() -> WebSession {
        Self::default()
    }
}
