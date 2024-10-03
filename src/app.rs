use crate::components::{PaymentPage, Sidenav};
use crate::pages::{
    AchievementsPage, ChallengesPage, LeaderboardPage, MarketplacePage, NotFoundPage, ResultsPage,
    SearchPage, SettingsPage,
};
use crate::utils::translation::{translation_config, LANGUAGE_KEY};
use crate::{
    components::Navigation,
    pages::{AboutPage, ChallengePage, HomePage, MapPage, ProfilePage},
    route::Route,
};
use gloo::storage::{LocalStorage, Storage};
use konnektoren_yew::i18n::I18nProvider;
use yew::prelude::*;
use yew_router::prelude::{BrowserRouter, Switch};

fn switch_main(route: Route) -> Html {
    match route {
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
        Route::NotFound => html! { <NotFoundPage /> },
        #[cfg(feature = "yew-preview")]
        Route::YewPreview => html! {<crate::pages::preview::PreviewPage />},
    }
}

fn update_language(query: &String) {
    let search_params = web_sys::UrlSearchParams::new_with_str(&query).unwrap();

    if let Some(lang) = search_params.get("lang") {
        LocalStorage::set(LANGUAGE_KEY, lang).unwrap_or_else(|err| {
            log::error!("Error setting language in local storage: {:?}", err);
        });
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let query = web_sys::window().unwrap().location().search();
    if let Ok(query) = query {
        update_language(&query);
    }

    let i18n_config = translation_config();

    html! {
        <I18nProvider config={i18n_config}>
            <BrowserRouter>
                <Sidenav />
                <Navigation />
                <Switch<Route> render={switch_main} />
            </BrowserRouter>
        </I18nProvider>
    }
}
