use crate::model::loader_error::LoaderError;
use crate::model::{LevelLoader, WebSession};
use konnektoren_core::challenges::{Challenge, ChallengeFactory};
use konnektoren_core::game::{GamePath, GameState};
use konnektoren_core::prelude::{Game, Session};

pub trait GameLoader<T> {
    fn load_game() -> Result<T, LoaderError>;
}

impl GameLoader<Game> for Game {
    fn load_game() -> Result<Game, LoaderError> {
        let game_paths = vec![
            GamePath::level_a1()?,
            GamePath::level_a2()?,
            GamePath::level_c1()?,
        ];
        let challenge_factory = ChallengeFactory::load_game()?;
        Ok(Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
        })
    }
}

impl GameLoader<GameState> for GameState {
    fn load_game() -> Result<GameState, LoaderError> {
        Ok(GameState {
            current_game_path: 0,
            current_challenge_index: 0,
            game: Game::load_game()?,
            challenge: Challenge::default(),
            current_task_index: 0,
        })
    }
}

impl GameLoader<ChallengeFactory> for ChallengeFactory {
    fn load_game() -> Result<ChallengeFactory, LoaderError> {
        let mut challenge_factory = ChallengeFactory::level_a1()?;
        challenge_factory
            .challenge_types
            .extend(ChallengeFactory::level_a2()?.challenge_types);
        challenge_factory
            .challenge_types
            .extend(ChallengeFactory::level_c1()?.challenge_types);

        Ok(challenge_factory)
    }
}

impl GameLoader<Session> for Session {
    fn load_game() -> Result<Session, LoaderError> {
        Ok(Session {
            id: Default::default(),
            player_profile: Default::default(),
            game_state: GameState::load_game()?,
        })
    }
}

impl GameLoader<WebSession> for WebSession {
    fn load_game() -> Result<WebSession, LoaderError> {
        Ok(WebSession {
            id: "websession".into(),
            session: Session::load_game()?,
        })
    }
}
