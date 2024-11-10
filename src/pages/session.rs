use crate::components::session::{session_login::LoginCallback, SessionPlayerProfile};
use crate::components::SessionLoginComp;
use konnekt_session::model::{Player, Role};
use konnektoren_yew::prelude::use_i18n;
use std::str::FromStr;
use uuid::Uuid;
use yew::prelude::*;

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

    let state = use_state(|| AppState::Login);

    let on_login = {
        let state = state.clone();

        Callback::from(move |(player, role, lobby_id, password): LoginCallback| {
            let lobby_id: Uuid = Uuid::from_str(&lobby_id).unwrap();
            state.set(AppState::Lobby((role, player, lobby_id, password.clone())));
        })
    };

    match (&*state).clone() {
        AppState::Login => {
            html! {
                <div class="session-page">
                    <h1>{ i18n.t("Session") }</h1>
                    <SessionLoginComp on_login={on_login} id={props.id.to_string()} />
                </div>
            }
        }
        AppState::Lobby((role, player, lobby_id, password)) => {
            html! {}
        }
    }
}
