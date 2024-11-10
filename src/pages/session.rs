use crate::components::session::{session_login::LoginCallback, SessionPlayerProfile};
use crate::components::{SessionLobbyComp, SessionLoginComp};
use crate::Route;
use konnekt_session::model::{Player, Role};
use konnektoren_yew::prelude::use_i18n;
use std::str::FromStr;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Default, Clone)]
enum AppState {
    #[default]
    Login,
    Lobby((Role, Player<SessionPlayerProfile>, Uuid, Option<String>)),
}

#[derive(Properties, PartialEq)]
pub struct SessionPageProps {
    pub id: String,
}

#[function_component(SessionPage)]
pub fn session_page(props: &SessionPageProps) -> Html {
    let i18n = use_i18n();
    let navigator = use_navigator().unwrap();

    let state = use_state(|| AppState::Login);
    let error = use_state(|| None::<String>);
    let id = match props.id.as_str() {
        "new" => "".to_string(),
        _ => props.id.clone(),
    };

    let on_login = {
        let state = state.clone();
        let error = error.clone();

        let navigator = navigator.clone();

        Callback::from(
            move |(player, role, lobby_id, password): LoginCallback| match Uuid::from_str(&lobby_id)
            {
                Ok(lobby_id) => {
                    state.set(AppState::Lobby((role, player, lobby_id, password.clone())));
                    navigator.push(&Route::Session {
                        id: lobby_id.to_string(),
                    });
                }
                Err(_) => {
                    error.set(Some("Invalid lobby ID".to_string()));
                    return;
                }
            },
        )
    };

    match (&*state).clone() {
        AppState::Login => {
            html! {
                <div class="session-page">
                    <h1>{ i18n.t("Session") }</h1>
                    <SessionLoginComp on_login={on_login} {id} />
                    <div>{(&*error).clone().unwrap_or_default()} </div>
                </div>
            }
        }
        AppState::Lobby((role, player, lobby_id, password)) => {
            html! {
                <div class="session-page">
                    <SessionLobbyComp
                        role={role.clone()}
                        player={player.clone()}
                        lobby_id={lobby_id.clone()}
                        password={password.clone()}
                    />
                </div>
            }
        }
    }
}
