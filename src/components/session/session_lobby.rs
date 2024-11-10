use super::{SessionChallenge, SessionChallengeComp, SessionPlayerProfile};

use konnekt_session::components::{LobbyComp, RunningActivityComp};
use konnekt_session::handler::{LocalLobbyCommandHandler, WebSocketLobbyCommandHandler};
use konnekt_session::model::{
    Activity, ActivityData, ActivityStatus, CommandError, Lobby, LobbyCommand, LobbyCommandHandler,
    Player, PlayerData, Role,
};
use konnektoren_core::game::Game;
use konnektoren_yew::prelude::use_game_state;
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
) -> Lobby<SessionPlayerProfile, SessionChallenge> {
    let mut lobby = Lobby::<SessionPlayerProfile, SessionChallenge>::new(player, password);

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

fn hash_lobby<P: PlayerData + Hash, A: ActivityData + Hash>(lobby: &Lobby<P, A>) -> u64 {
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
    let game_state = use_game_state();
    let game = game_state.game.clone();
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

    let last_event = use_state(|| 0);

    // Create WebSocket handler
    let websocket_handler = use_state(|| {
        let local_handler = LocalLobbyCommandHandler::<SessionPlayerProfile, SessionChallenge>::new(
            |data: &str| serde_json::from_str(data).expect("Failed to deserialize player data"),
            |data: &str| serde_json::from_str(data).expect("Failed to deserialize challenge data"),
        );

        let update_ui = Callback::from(
            move |lobby: Lobby<SessionPlayerProfile, SessionChallenge>| {
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
                "Participant" => Role::Participant,
                "Observer" => Role::Observer,
                _ => Role::Participant,
            };
            role.set(selected_role);
        }
    };

    // Get current lobby state
    let current_lobby = (&*lobby.borrow()).clone();

    html! {
        <div>
            <div>{"Connected to lobby: "}{lobby_id.to_string()}</div>
            <select onchange={on_change} value={role.to_string()}>
                <option value="Admin">{"Admin"}</option>
                <option value="Participant">{"Participant"}</option>
                <option value="Observer">{"Observer"}</option>
            </select>
            <LobbyComp<SessionPlayerProfile, SessionChallenge>
                lobby={current_lobby.clone()}
                role={*role}
                on_command={on_command.clone()}
                {on_error}
            />
            <RunningActivityComp<SessionChallenge, SessionChallengeComp>
                activities={current_lobby.activities.clone()}
                role={*role}
                on_command={on_command}
            />
        </div>
    }
}
