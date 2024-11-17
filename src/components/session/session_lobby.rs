use crate::model::LevelLoader;

use super::{
    SessionChallenge, SessionChallengeComp, SessionChallengeResult, SessionChallengeResultComp,
    SessionPlayerProfile,
};
use konnekt_session::components::{ActivityResultDetailComp, LobbyComp, RunningActivityComp};
use konnekt_session::handler::{LocalLobbyCommandHandler, WebSocketLobbyCommandHandler};
use konnekt_session::model::{
    Activity, ActivityResult, ActivityResultTrait, ActivityStatus, ActivityTrait, CommandError,
    Lobby, LobbyCommand, LobbyCommandHandler, Player, PlayerId, PlayerTrait, Role,
};
use konnektoren_core::game::Game;
use konnektoren_yew::components::SharePageComp;
use serde::Serialize;
use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use uuid::Uuid;
use web_sys::Event;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

const API_URL: &str = "wss://api.konnektoren.help/session";

fn init_lobby(
    game: Game,
    player: Player<SessionPlayerProfile>,
    password: Option<String>,
) -> Lobby<SessionPlayerProfile, SessionChallenge, SessionChallengeResult> {
    let mut lobby = Lobby::<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>::new(
        player, password,
    );

    let challenges = game
        .game_paths
        .iter()
        .map(|path| path.challenge_ids())
        .flatten()
        .map(|id| SessionChallenge {
            id: id.to_string(),
            name: id.to_string(),
        });

    for challenge in challenges {
        let activity = Activity {
            id: challenge.id.clone(),
            status: ActivityStatus::NotStarted,
            data: challenge,
        };
        lobby.add_activity(activity);
    }

    lobby
}

fn hash_lobby<
    P: PlayerTrait + Hash,
    A: ActivityTrait + Hash,
    AR: ActivityResultTrait + Hash + Serialize,
>(
    lobby: &Lobby<P, A, AR>,
) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    lobby.hash(&mut hasher);
    hasher.finish()
}

#[derive(Properties, PartialEq)]
pub struct LobbyProps {
    pub role: Role,
    pub player: Player<SessionPlayerProfile>,
    pub lobby_id: Uuid,
    pub password: Option<String>,
}

#[function_component(SessionLobbyComp)]
pub fn lobby_page(props: &LobbyProps) -> Html {
    let game = Game::level_a1().unwrap();
    let role = use_state(|| props.player.role.clone());
    let lobby_id = use_state(|| props.lobby_id.clone());

    let player = use_state(|| RefCell::new(props.player.clone()));
    let lobby = use_state(|| {
        RefCell::new(init_lobby(
            game,
            props.player.clone(),
            props.password.clone(),
        ))
    });

    let selected_activity_result =
        use_state(|| None::<(PlayerId, ActivityResult<SessionChallengeResult>)>);
    let last_event = use_state(|| 0);

    // Create WebSocket handler
    let websocket_handler = use_state(|| {
        let local_handler = LocalLobbyCommandHandler::<
            SessionPlayerProfile,
            SessionChallenge,
            SessionChallengeResult,
        >::new(
            |data: &str| serde_json::from_str(data).expect("Failed to deserialize player data"),
            |data: &str| serde_json::from_str(data).expect("Failed to deserialize challenge data"),
            |data: &str| {
                serde_json::from_str(data).expect("Failed to deserialize challenge result data")
            },
        );

        let update_ui = Callback::from(
            move |lobby: Lobby<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>| {
                last_event.set(hash_lobby(&lobby));
            },
        );

        let player = player.clone();
        let password = props.password.clone();

        WebSocketLobbyCommandHandler::new(
            &API_URL,
            *lobby_id,
            player.clone(),
            password,
            local_handler.clone(),
            lobby.clone(),
            update_ui,
        )
    });

    let on_command = {
        let handler = websocket_handler.clone();
        Callback::from(move |command: LobbyCommand| {
            if let Err(err) = handler.send_command(command) {
                log::info!("Command error: {:?}", err);
            }
        })
    };

    let on_error = {
        Callback::from(move |err: CommandError| {
            log::error!("Command error: {:?}", err);
        })
    };

    let on_change = {
        let role = role.clone();
        move |e: Event| {
            let target = e.target_unchecked_into::<HtmlSelectElement>();
            let value = target.value();
            let selected_role = match value.as_str() {
                "Admin" => Role::Admin,
                "Participant" => Role::Player,
                "Observer" => Role::Observer,
                _ => Role::Player,
            };
            role.set(selected_role);
        }
    };

    // Get current lobby state
    let current_lobby = (&*lobby.borrow()).clone();

    let player_id = (&*player.borrow()).id;

    let activity_result_detail = {
        let lobby = lobby.borrow().clone();
        match selected_activity_result.as_ref() {
            Some((player_id, result)) => {
                let player = lobby
                    .participants
                    .iter()
                    .find(|p| p.id == *player_id)
                    .unwrap()
                    .clone();
                html! {
                    <ActivityResultDetailComp<_, _, SessionChallengeResultComp>
                        {player}
                        result={result.clone()}
                    />
                }
            }
            None => {
                html! {}
            }
        }
    };

    let on_activity_result_select = {
        let selected_activity_result = selected_activity_result.clone();

        Callback::from(
            move |(player_id, result): (PlayerId, ActivityResult<SessionChallengeResult>)| {
                log::info!("on_activity_result_select");
                selected_activity_result.set(Some((player_id, result.clone())));
            },
        )
    };

    html! {
        <div>
            <SharePageComp />
            <select onchange={on_change} value={role.to_string()}>
                <option value="Admin">{"Admin"}</option>
                <option value="Participant">{"Participant"}</option>
                <option value="Observer">{"Observer"}</option>
            </select>
            <LobbyComp<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>
                lobby={current_lobby.clone()}
                role={*role}
                on_command={on_command.clone()}
                {on_error}
                {on_activity_result_select}
            />
            {activity_result_detail}
            <RunningActivityComp<SessionChallenge, SessionChallengeComp>
                {player_id}
                activities={current_lobby.activities.clone()}
                role={*role}
                on_command={on_command}
            />
        </div>
    }
}
