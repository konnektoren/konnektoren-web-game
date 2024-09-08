use crate::route::Route;
use konnektoren_core::game::GamePath;
use konnektoren_yew::components::game_map::ChallengeIndex;
use konnektoren_yew::prelude::BrowserCoordinate;
use web_sys::window;
use yew::callback;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub game_path: GamePath,
    pub current_challenge: usize,
    #[prop_or_default]
    pub on_select_challenge: Option<Callback<(Option<ChallengeIndex>, BrowserCoordinate)>>,
}

#[function_component(ChallengeNavigationComp)]
pub fn challenge_navigation_comp(props: &Props) -> Html {
    let current_challenge = props.current_challenge;
    let challenge = props.game_path.challenges.get(props.current_challenge);
    let challenge_config = match challenge {
        Some(challenge) => challenge,
        None => return html! {<div></div>},
    };

    let navigator = use_navigator().unwrap();
    let is_unlocked = super::challenge_info::challenge_unlocked(&challenge_config);

    let on_new = {
        let id = challenge_config.id.clone();
        if is_unlocked {
            Callback::from(move |_| {
                let id = id.clone();
                log::info!("Challenge selected: {}", id);
                navigator.push(&Route::Challenge { id });
            })
        } else {
            Callback::from(move |_| {
                log::info!("Challenge selected: {}", id);
                window()
                    .unwrap()
                    .alert_with_message("Challenge is locked")
                    .expect("alert failed");
            })
        }
    };

    let on_previous = {
        let on_select_challenge = props.on_select_challenge.clone();

        callback::Callback::from(move |_| {
            on_select_challenge.as_ref().map(|callback| {
                let previous_challenge = if current_challenge > 0 {
                    Some(current_challenge - 1)
                } else {
                    None
                };
                callback.emit((previous_challenge, BrowserCoordinate(0.0, 0.0)));
            });
        })
    };

    let on_next = {
        let on_select_challenge = props.on_select_challenge.clone();
        let challenges: usize = props.game_path.challenges.len();
        callback::Callback::from(move |_| {
            on_select_challenge.as_ref().map(|callback| {
                let next_challenge = if current_challenge < challenges - 1 {
                    Some(current_challenge + 1)
                } else {
                    None
                };
                callback.emit((next_challenge, BrowserCoordinate(0.0, 0.0)));
            });
        })
    };

    html! {
        <div class="challenge-navigation">
            <button class="previous-challenge fa-solid fa-arrow-left-long" onclick={on_previous}></button>
            {
                if is_unlocked {
                    html! {<i class="fa-solid fa-unlock"></i>}
                } else {
                    html! {<i class="fa-solid fa-lock"></i>}
                }
            }
            <div class="selected-challenge" onclick={on_new}>{&challenge_config.name}</div>
            <button class="next-challenge fa-solid fa-arrow-right-long" onclick={on_next}></button>
        </div>
    }
}
