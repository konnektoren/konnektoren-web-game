use crate::model::LevelLoader;

use super::{
    SessionChallenge, SessionChallengeComp, SessionChallengeResult, SessionChallengeResultComp,
    SessionPlayerProfile,
};
use konnekt_session::components::{
    use_lobby, use_lobby_handler, ActivityResultDetailComp, LobbyComp, LobbyProvider,
    LobbyProviderConfig, RunningActivityComp,
};
use konnekt_session::handler::NetworkHandler;
use konnekt_session::model::{
    Activity, ActivityResult, ActivityResultTrait, ActivityStatus, ActivityTrait, CommandError,
    Lobby, LobbyCommand, LobbyCommandHandler, LobbyId, Player, PlayerId, PlayerTrait, Role,
    TransportType,
};
use konnektoren_core::game::Game;
use konnektoren_yew::components::SharePageComp;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;
use web_sys::Event;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

const API_URL: &str = "wss://api.konnektoren.help/session";

fn init_lobby(
    game: Game,
    lobby_id: LobbyId,
    player: Player<SessionPlayerProfile>,
    password: Option<String>,
) -> Lobby<SessionPlayerProfile, SessionChallenge, SessionChallengeResult> {
    let mut lobby =
        Lobby::<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>::new_with_id(
            lobby_id, player, password,
        );

    let challenges = game
        .game_paths
        .iter()
        .flat_map(|path| path.challenge_ids())
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
    pub lobby_id: LobbyId,
    pub password: Option<String>,
}

#[function_component(SessionLobbyComp)]
pub fn lobby_page(props: &LobbyProps) -> Html {
    let game = Game::level_a1().unwrap();
    let role = use_state(|| props.player.role);
    let lobby_id = use_state(|| props.lobby_id);

    let on_change = {
        let role = role.clone();
        move |e: Event| {
            let target = e.target_unchecked_into::<HtmlSelectElement>();
            let value = target.value();
            let selected_role = match value.as_str() {
                "Admin" => Role::Admin,
                "Player" => Role::Player,
                _ => Role::Player,
            };
            role.set(selected_role);
        }
    };

    let transport = TransportType::WebSocket(API_URL.to_string());

    let lobby_provider_config = LobbyProviderConfig {
        transport,
        player: props.player.clone(),
        lobby: init_lobby(
            game,
            props.lobby_id,
            props.player.clone(),
            props.password.clone(),
        ),
        role: *role,
        debug: false,
    };

    html! {
        <div>
            <SharePageComp />
            <LobbyProvider<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>
                config={lobby_provider_config}
            >
            <div>{"Connected to lobby: "}{lobby_id.to_string()}</div>
            <select onchange={on_change} value={role.to_string()}>
                <option value="Admin">{"Admin"}</option>
                <option value="Participant">{"Participant"}</option>
                <option value="Observer">{"Observer"}</option>
            </select>

            <LobbyInnerComp player={props.player.clone()} role={*role} />

            </LobbyProvider<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LobbyInnerProps {
    pub role: Role,
    pub player: Player<SessionPlayerProfile>,
}

#[function_component(LobbyInnerComp)]
pub fn lobby_inner_comp(props: &LobbyInnerProps) -> Html {
    let lobby = use_lobby();
    let lobby_handler =
        use_lobby_handler::<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>();

    let current_lobby = (*lobby).clone();

    let selected_activity_result =
        use_state(|| None::<(PlayerId, ActivityResult<SessionChallengeResult>)>);

    let on_command = {
        let handler = lobby_handler.clone();
        Callback::from(move |command: LobbyCommand| {
            let handler: NetworkHandler<_, _, _> = (*handler).clone();
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

    let activity_result_detail = {
        let lobby = lobby.clone();
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
        <LobbyComp<SessionPlayerProfile, SessionChallenge, SessionChallengeResult>
                lobby={current_lobby.clone()}
                role={props.role}
                on_command={on_command.clone()}
                {on_error}
                {on_activity_result_select}
            />
            {activity_result_detail}
            <RunningActivityComp<SessionChallenge, SessionChallengeComp>
                player_id={props.player.id}
                activities={current_lobby.activities.clone()}
                role={props.role}
                on_command={on_command}
            />
        </div>
    }
}
