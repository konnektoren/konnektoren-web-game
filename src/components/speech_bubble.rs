use crate::components::Roulette;
use crate::model::WebSession;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::ProfileConfigComponent;
use konnektoren_yew::prelude::SelectLanguage;
use konnektoren_yew::prelude::SelectLevelComp;
use yew::prelude::*;

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

    let web_session = WebSession::default();
    let game_paths = web_session.session.game_state.game.game_paths.clone();
    let current_level = use_state(|| web_session.session.game_state.current_game_path);

    let handle_switch_level = {
        let web_session = web_session.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let mut web_session = web_session.clone();
            web_session.session.game_state.current_game_path = level;
            web_session.save().unwrap();
            current_level.set(level);
        })
    };

    html! {
        <div class="speech-bubble">
            <p>{ i18n.t("I am your personal german learning assistant.") }</p>
            <p>{ i18n.t("Before we start tell me something about yourself.") }</p>
            <ProfileConfigComponent />
            <p>{ i18n.t("Now select your level of german.") }</p>
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
            <SelectLanguage />
            <button onclick={props.on_state_change.reform(|_| ())}>{ "Start Learning" }</button>
        </div>
    }
}

#[function_component(SpeechBubbleLearning)]
fn speech_bubble_learning() -> Html {
    let i18n = use_i18n();
    html! {
        <div class="speech-bubble">
            <p>{ i18n.t("Select a german topic to learn with me.") }</p>
            <Roulette />
        </div>
    }
}
