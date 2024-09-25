use crate::model::loader_error::LoaderError;
use crate::model::product_repository::ProductRepository;
use crate::model::WebSession;
use konnektoren_core::game::{Game, GamePath, GameState};
use konnektoren_core::prelude::{Challenge, ChallengeFactory, ChallengeType, Session};

pub trait LevelLoader<T> {
    fn level_a1() -> Result<T, LoaderError>;
    fn level_a2() -> Result<T, LoaderError>;
    fn level_b1() -> Result<T, LoaderError>;
    fn level_c1() -> Result<T, LoaderError>;

    fn custom_level() -> Result<T, LoaderError>;
}

impl LevelLoader<ChallengeFactory> for ChallengeFactory {
    fn level_a1() -> Result<ChallengeFactory, LoaderError> {
        let articles: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/articles.yml"))?;
        let articles_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/articles_info.yml"))?;
        let reflexivpronouns: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/reflexivpronouns.yml"))?;
        let reflexivpronouns_info: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/reflexivpronouns_info.yml"
        ))?;
        let personalpronouns: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/personal_pronouns.yml"))?;
        let personalpronouns_info: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/personal_pronouns_info.yml"
        ))?;
        let connectives: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/konnektoren.yml"))?;
        let connectives_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/connectives_info.yml"))?;

        Ok(ChallengeFactory {
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
        })
    }

    fn level_a2() -> Result<ChallengeFactory, LoaderError> {
        let articles: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/articles.yml"))?;
        let articles_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/articles_info.yml"))?;

        Ok(ChallengeFactory {
            challenge_types: vec![articles, articles_info],
        })
    }

    fn level_b1() -> Result<ChallengeFactory, LoaderError> {
        let custom_past_tenses: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/custom_past_tenses.yml"))?;
        let custom_verbs_prepositions: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/custom_verbs_prepositions.yml"
        ))?;
        let custom_interrogative_particles: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/custom_interrogative_particles.yml"
        ))?;

        Ok(ChallengeFactory {
            challenge_types: vec![
                custom_past_tenses,
                custom_verbs_prepositions,
                custom_interrogative_particles,
            ],
        })
    }

    fn level_c1() -> Result<ChallengeFactory, LoaderError> {
        let connectives: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/konnektoren.yml"))?;
        let connectives_info: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/connectives_info.yml"))?;
        let custom_articles: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/custom_articles.yml"))?;
        let custom_verbs: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/custom_verbs.yml"))?;
        let custom_perfect_tense: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/custom_perfect_tense.yml"
        ))?;
        let custom_group_of_nouns: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/custom_group_of_nouns.yml"
        ))?;
        let custom_negation: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/custom_negation.yml"))?;
        let custom_haupt_neben_satze: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/custom_haupt_neben_satze.yml"
        ))?;
        let custom_verbs_mit_dativ_accusativ: ChallengeType = serde_yaml::from_str(include_str!(
            "../assets/challenges/custom_verbs_mit_dativ_accusativ.yml"
        ))?;
        let custom_konjunktiv2: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/custom_konjunktiv2.yml"))?;
        let custom_zeitangaben: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/custom_zeitangaben.yml"))?;
        let custom_casus: ChallengeType =
            serde_yaml::from_str(include_str!("../assets/challenges/custom_casus.yml"))?;

        Ok(ChallengeFactory {
            challenge_types: vec![
                connectives,
                connectives_info,
                custom_articles,
                custom_verbs,
                custom_perfect_tense,
                custom_group_of_nouns,
                custom_negation,
                custom_haupt_neben_satze,
                custom_verbs_mit_dativ_accusativ,
                custom_konjunktiv2,
                custom_zeitangaben,
                custom_casus,
            ],
        })
    }

    fn custom_level() -> Result<ChallengeFactory, LoaderError> {
        Ok(ChallengeFactory {
            challenge_types: vec![],
        })
    }
}

impl LevelLoader<GamePath> for GamePath {
    fn level_a1() -> Result<GamePath, LoaderError> {
        let game_path: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_a1.yml"))?;
        Ok(game_path)
    }

    fn level_a2() -> Result<GamePath, LoaderError> {
        let game_path: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_a2.yml"))?;
        Ok(game_path)
    }

    fn level_b1() -> Result<GamePath, LoaderError> {
        let game_path: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_b1.yml"))?;
        Ok(game_path)
    }

    fn level_c1() -> Result<GamePath, LoaderError> {
        let game_path: GamePath =
            serde_yaml::from_str(include_str!("../assets/challenges/level_c1.yml"))?;
        Ok(game_path)
    }

    fn custom_level() -> Result<GamePath, LoaderError> {
        Ok(ProductRepository::new().fetch_custom_level().unwrap())
    }
}

impl LevelLoader<Game> for Game {
    fn level_a1() -> Result<Game, LoaderError> {
        let game_paths = vec![GamePath::level_a1()?];
        let challenge_factory = ChallengeFactory::level_a1()?;
        Ok(Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        })
    }

    fn level_a2() -> Result<Game, LoaderError> {
        let game_paths = vec![GamePath::level_a2()?];
        let challenge_factory = ChallengeFactory::level_a2()?;
        Ok(Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        })
    }

    fn level_b1() -> Result<Game, LoaderError> {
        let game_paths = vec![GamePath::level_b1()?];
        let challenge_factory = ChallengeFactory::level_b1()?;
        Ok(Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        })
    }

    fn level_c1() -> Result<Game, LoaderError> {
        let game_paths = vec![GamePath::level_c1()?];
        let challenge_factory = ChallengeFactory::level_c1()?;
        Ok(Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        })
    }

    fn custom_level() -> Result<Game, LoaderError> {
        let game_paths = vec![GamePath::custom_level()?];
        let challenge_factory = ChallengeFactory::custom_level()?;
        Ok(Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        })
    }
}

impl LevelLoader<GameState> for GameState {
    fn level_a1() -> Result<GameState, LoaderError> {
        Ok(GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::level_a1()?,
            challenge: Challenge::default(),
            current_task_index: 0,
        })
    }

    fn level_a2() -> Result<GameState, LoaderError> {
        Ok(GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::level_a2()?,
            challenge: Challenge::default(),
            current_task_index: 0,
        })
    }

    fn level_b1() -> Result<GameState, LoaderError> {
        Ok(GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::level_b1()?,
            challenge: Challenge::default(),
            current_task_index: 0,
        })
    }

    fn level_c1() -> Result<GameState, LoaderError> {
        Ok(GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::level_c1()?,
            challenge: Challenge::default(),
            current_task_index: 0,
        })
    }

    fn custom_level() -> Result<GameState, LoaderError> {
        Ok(GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::custom_level()?,
            challenge: Challenge::default(),
            current_task_index: 0,
        })
    }
}

impl LevelLoader<Session> for Session {
    fn level_a1() -> Result<Session, LoaderError> {
        Ok(Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_a1()?,
        })
    }

    fn level_a2() -> Result<Session, LoaderError> {
        Ok(Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_a2()?,
        })
    }

    fn level_b1() -> Result<Session, LoaderError> {
        Ok(Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_b1()?,
        })
    }

    fn level_c1() -> Result<Session, LoaderError> {
        Ok(Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::level_c1()?,
        })
    }

    fn custom_level() -> Result<Session, LoaderError> {
        Ok(Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::custom_level()?,
        })
    }
}

impl LevelLoader<WebSession> for WebSession {
    fn level_a1() -> Result<WebSession, LoaderError> {
        Ok(WebSession {
            id: "websession".into(),
            session: Session::level_a1()?,
        })
    }

    fn level_a2() -> Result<WebSession, LoaderError> {
        Ok(WebSession {
            id: "websession".into(),
            session: Session::level_a2()?,
        })
    }

    fn level_b1() -> Result<WebSession, LoaderError> {
        Ok(WebSession {
            id: "websession".into(),
            session: Session::level_b1()?,
        })
    }

    fn level_c1() -> Result<WebSession, LoaderError> {
        Ok(WebSession {
            id: "websession".into(),
            session: Session::level_c1()?,
        })
    }

    fn custom_level() -> Result<WebSession, LoaderError> {
        Ok(WebSession {
            id: "websession".into(),
            session: Session::custom_level()?,
        })
    }
}
