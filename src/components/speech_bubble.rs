use crate::components::Roulette;
use crate::Route;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::use_session;
use konnektoren_yew::prelude::use_session_repository;
use konnektoren_yew::prelude::SelectLanguage;
use konnektoren_yew::prelude::SelectLevelComp;
use konnektoren_yew::repository::SESSION_STORAGE_KEY;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum State {
    NewUser,
    Learning,
}

#[function_component(SpeechBubble)]
pub fn speech_bubble() -> Html {
    let state = use_state(|| State::NewUser);

    let on_state_change = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(State::Learning);
        })
    };

    match *state {
        State::NewUser => html! {
            <SpeechBubbleNewUser on_state_change={on_state_change} />
        },
        State::Learning => html! {
            <SpeechBubbleLearning />
        },
    }
}

#[derive(Properties, PartialEq, Clone)]
struct SpeechBubbleNewUserProps {
    on_state_change: Callback<()>,
}

#[function_component(SpeechBubbleNewUser)]
fn speech_bubble_new_user(props: &SpeechBubbleNewUserProps) -> Html {
    let i18n = use_i18n();

    let session = use_session();
    let session_repository = use_session_repository();

    let game_state = session.read().unwrap().game_state.clone();
    let game_paths = game_state.game.game_paths.clone();
    let current_level = use_state(|| game_state.current_game_path);

    let handle_switch_level = {
        let session = session.clone();
        let session_repository = session_repository.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let session = session.clone();
            let session_repository = session_repository.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let session = session.clone();
                let mut new_session = session.read().unwrap().clone();
                new_session.game_state.current_game_path = level;
                session_repository
                    .save_session(SESSION_STORAGE_KEY, &new_session)
                    .await
                    .unwrap();
                let mut session_guard = session.write().unwrap();
                *session_guard = new_session;
            });
            current_level.set(level);
        })
    };

    html! {
        <div class="speech-bubble">
            <p>{ i18n.t("I am your personal german learning assistant.") }</p>
            <p>{ i18n.t("Before we start tell me something about yourself.") }</p>
            <p>{ i18n.t("Now select your level of german.") }</p>
            <div class="speech-bubble__level-selection">
                <span class="speech-bubble__flag-emoji">{ "🇩🇪" }</span>
                <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
            </div>
            <SelectLanguage />
            <button class="speech-bubble__start-button" onclick={props.on_state_change.reform(|_| ())}>{ i18n.t("Start Learning") }</button>
        </div>
    }
}

#[function_component(SpeechBubbleLearning)]
fn speech_bubble_learning() -> Html {
    let i18n = use_i18n();

    let session = use_session();
    let current_level = use_state(|| session.read().unwrap().game_state.current_game_path);

    let navigator = use_navigator().unwrap();

    {
        let current_level = current_level.clone();
        use_effect_with(current_level.clone(), move |current_level| {
            if **current_level != 0 {
                navigator.push(&Route::Challenges);
            }
        });
    }
    html! {
        <div class="speech-bubble">
            <p>{ i18n.t("Select a german topic to learn with me.") }</p>
            <Roulette />
        </div>
    }
}
