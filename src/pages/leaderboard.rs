use crate::components::LeaderboardComp;
use crate::model::WebSession;
use gloo::utils::document;
use konnektoren_yew::i18n::use_i18n;
use konnektoren_yew::prelude::SelectLevelComp;
use yew::prelude::*;

#[function_component(LeaderboardPage)]
pub fn leaderboard_page() -> Html {
    let i18n = use_i18n();
    let title = format!("Konnektoren - {}", i18n.t("Leaderboard"));
    use_effect(move || {
        document().set_title(&title);
        || ()
    });

    let mut web_session = WebSession::default();
    web_session.load();
    let game_paths = web_session.session.game_state.game.game_paths.clone();
    let current_level = use_state(|| web_session.session.game_state.current_game_path);

    let current_level_id = game_paths[*current_level].id.clone();

    let switch_level = {
        let web_session = web_session.clone();
        let current_level = current_level.clone();
        Callback::from(move |level: usize| {
            let mut web_session = web_session.clone();
            web_session.session.game_state.current_game_path = level;
            web_session.save();
            current_level.set(level);
        })
    };

    html! {
        <div class="leaderboard-page">
            <h1>{i18n.t("Leaderboard")}</h1>
            <SelectLevelComp levels={game_paths.clone()} current={*current_level} on_select={switch_level} />
            <LeaderboardComp challenge={current_level_id} />
        </div>
    }
}
