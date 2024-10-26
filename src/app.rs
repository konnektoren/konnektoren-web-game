use crate::components::{PaymentPage, Sidenav};
use crate::model::SessionInitializerImpl;
use crate::pages::{
    AchievementsPage, ChallengesPage, LeaderboardPage, MarketplacePage, NotFoundPage, ResultsPage,
    SearchPage, SettingsPage,
};
use crate::utils::translation::{translation_config, LANGUAGE_KEY};
use crate::{
    components::Navigation,
    pages::{AboutPage, ChallengePage, HomePage, MapPage, ProfilePage},
    Route,
};
use gloo::storage::{LocalStorage as GlooStorage, Storage};
use konnektoren_core::controller::{ControllerPlugin, DebugPlugin};
use konnektoren_yew::i18n::I18nProvider;
use konnektoren_yew::prelude::repository_provider::create_repositories;
use konnektoren_yew::providers::{use_game_controller, GameControllerProvider, RepositoryProvider};
use konnektoren_yew::repository::LocalStorage;
use std::sync::Arc;
use yew::prelude::*;
use yew_router::prelude::Switch;

fn switch_main(route: Route) -> Html {
    match route {
        Route::Root => html! {<HomePage />},
        Route::About => html! {<AboutPage /> },
        Route::Achievements => html! {<AchievementsPage />},
        Route::Challenge { id } => html! {<ChallengePage {id} />},
        Route::Challenges => html! {<ChallengesPage />},
        Route::Home => html! {<HomePage />},
        Route::Ads => html! {<HomePage />},
        Route::Leaderboard => html! {<LeaderboardPage />},
        Route::Marketplace => html! {<MarketplacePage />},
        Route::Map => html! {<MapPage />},
        Route::Profile => html! {<ProfilePage />},
        Route::Results { code } => html! {<ResultsPage { code } />},
        Route::Payment => html! {<PaymentPage />},
        Route::Search => html! {<SearchPage />},
        Route::SearchWithQuery { query } => html! {<SearchPage search_query={query} />},
        Route::Settings => html! {<SettingsPage />},
        Route::NotFound | Route::Welcome => html! { <NotFoundPage /> },
        #[cfg(feature = "yew-preview")]
        Route::YewPreview => html! {<crate::pages::preview::PreviewPage />},
    }
}

fn update_language(query: &String) {
    let search_params = web_sys::UrlSearchParams::new_with_str(&query).unwrap();

    if let Some(lang) = search_params.get("lang") {
        GlooStorage::set(LANGUAGE_KEY, lang).unwrap_or_else(|err| {
            log::error!("Error setting language in local storage: {:?}", err);
        });
    }
}

#[function_component(InitApp)]
pub fn init_app() -> Html {
    let game_controller = use_game_controller();

    use_effect_with((), move |_| {
        let game_controller = game_controller.clone();
        let debug_plugin = Arc::new(DebugPlugin::new(Arc::new(log::logger())));
        debug_plugin.load(game_controller.controller).unwrap();
    });

    html! {
        <div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let query = web_sys::window().unwrap().location().search();
    if let Ok(query) = query {
        update_language(&query);
    }

    let i18n_config = translation_config();
    let storage = LocalStorage::new(None);
    let session_initilizer = SessionInitializerImpl;
    let repository_config = create_repositories(storage, Arc::new(session_initilizer));

    html! {
        <RepositoryProvider config={repository_config}>
        <I18nProvider config={i18n_config}>

        <GameControllerProvider>
            <InitApp />
            <Sidenav />
            <Navigation />
            <Switch<Route> render={switch_main} />
        </GameControllerProvider>
        </I18nProvider>
        </RepositoryProvider>
    }
}
