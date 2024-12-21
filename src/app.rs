use crate::components::PaymentPage;
use crate::model::SessionInitializerImpl;
use crate::pages::session::SessionPage;
#[cfg(feature = "backup")]
use crate::pages::BackupPage;
use crate::pages::{
    AchievementsPage, ChallengesPage, LeaderboardPage, MarketplacePage, NotFoundPage, ResultsPage,
    SearchPage, SettingsPage,
};
use crate::utils::translation::translation_config;
use crate::{
    components::Navigation,
    pages::{AboutPage, ChallengePage, HomePage, MapPage, ProfilePage},
    Route,
};
use konnektoren_core::controller::{ControllerPlugin, DebugPlugin};
use konnektoren_yew::i18n::I18nProvider;
use konnektoren_yew::prelude::repository_provider::create_repositories;
use konnektoren_yew::providers::{
    use_game_controller, DesignProvider, GameControllerProvider, RepositoryProvider, ThemeProvider,
};
use konnektoren_yew::repository::LocalStorage;
use std::sync::Arc;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::Switch;

fn switch_main(route: Route) -> Html {
    match route {
        Route::Root => html! {<HomePage />},
        Route::About => html! {<AboutPage /> },
        Route::Achievements => html! {<AchievementsPage />},
        #[cfg(feature = "backup")]
        Route::Backup => html! {<BackupPage />},
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
        Route::Session { id } => html! {<SessionPage {id} />},
        Route::Settings => html! {<SettingsPage />},
        Route::NotFound | Route::Welcome => html! { <NotFoundPage /> },
        #[cfg(feature = "yew-preview")]
        Route::YewPreview => html! {<crate::pages::preview::PreviewPage />},
    }
}

#[function_component(InitApp)]
pub fn init_app() -> Html {
    let game_controller = use_game_controller();

    use_effect_with((), move |_| {
        if let Some(window) = window() {
            if let Ok(location) = window.location().search() {
                // Remove the leading '?' and split into key-value pairs
                let query_string = location.trim_start_matches('?');
                log::info!("Query parameters: {}", query_string);

                // Parse and log individual parameters
                for param in query_string.split('&') {
                    if let Some((key, value)) = param.split_once('=') {
                        log::info!("Parameter: {} = {}", key, value);
                    }
                }
            }
        }

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
    let i18n_config = translation_config();
    let storage = LocalStorage::new(None);
    let session_initilizer = SessionInitializerImpl;
    let repository_config = create_repositories(storage, Arc::new(session_initilizer));

    html! {
        <RepositoryProvider config={repository_config}>
        <I18nProvider config={i18n_config}>
        <DesignProvider>
        <ThemeProvider>
        <GameControllerProvider>
            <InitApp />
            <Navigation />
            <Switch<Route> render={switch_main} />
        </GameControllerProvider>
        </ThemeProvider>
        </DesignProvider>
        </I18nProvider>
        </RepositoryProvider>
    }
}
