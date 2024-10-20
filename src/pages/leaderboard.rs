use crate::components::LeaderboardComp;
use gloo::utils::document;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::SelectLevelComp;
use konnektoren_yew::providers::{use_session, use_session_repository};
use konnektoren_yew::repository::SESSION_STORAGE_KEY;
use yew::prelude::*;

#[function_component(LeaderboardPage)]
pub fn leaderboard_page() -> Html {
    let i18n = use_i18n();
    let title = format!("Konnektoren - {}", i18n.t("Leaderboard"));
    use_effect(move || {
        document().set_title(&title);
        || ()
    });

    let session = use_session();
    let session_repository = use_session_repository();

    let game_paths = session.game_state.game.game_paths.clone();
    let current_level = use_state(|| session.game_state.current_game_path);

    let current_level_id = game_paths[*current_level].id.clone();

    let handle_switch_level = {
        let session = session.clone();
        let session_repository = session_repository.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let session = session.clone();
            let session_repository = session_repository.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut new_session = (&*session).clone();
                new_session.game_state.current_game_path = level;
                session_repository
                    .save_session(SESSION_STORAGE_KEY, &new_session)
                    .await
                    .unwrap();
                session.set(new_session);
            });
            current_level.set(level);
        })
    };

    html! {
        <div class="leaderboard-page">
            <h1>{i18n.t("Leaderboard")}</h1>
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={handle_switch_level} />
            <LeaderboardComp challenge={current_level_id} />
        </div>
    }
}
