use crate::model::GameLoader;
use crate::model::LevelLoader;
use crate::model::LoaderError;
use konnektoren_core::game::Game;
use konnektoren_core::session::Session;

pub fn merge_with_new_game(session: &Session) -> Result<Session, LoaderError> {
    let new_game = Game::load_game()?;
    let mut new_session = Session::level_a1()?;

    new_session.game_state.game.game_paths = new_game.game_paths;
    new_session.game_state.game.challenge_factory = new_game.challenge_factory;

    new_session.player_profile = session.player_profile.clone();
    new_session.game_state.current_game_path = session.game_state.current_game_path;
    new_session.game_state.current_challenge_index = session.game_state.current_challenge_index;
    new_session.game_state.current_task_index = session.game_state.current_task_index;

    new_session
        .game_state
        .game
        .challenge_history
        .merge(&session.game_state.game.challenge_history);

    Ok(new_session)
}
