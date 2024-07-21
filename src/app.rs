use crate::components::Sidenav;
use crate::pages::{LeaderboardPage, ResultsPage};
use crate::{
    components::Navigation,
    pages::{AboutPage, ChallengePage, HomePage, MapPage, ProfilePage},
    route::Route,
    utils::translation::{translations, LANGUAGES},
};
use yew::prelude::*;
use yew_i18n::I18nProvider;
use yew_router::prelude::{BrowserRouter, Switch};

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<AboutPage /> },
        Route::Challenge { id } => html! {<ChallengePage {id} />},
        Route::Home => html! {<HomePage />},
        Route::Leaderboard => html! {<LeaderboardPage />},
        Route::Map => html! {<MapPage />},
        Route::Profile => html! {<ProfilePage />},
        Route::Results { code } => html! {<ResultsPage { code } />},
        #[cfg(feature = "yew-preview")]
        Route::YewPreview => html! {<crate::pages::preview::PreviewPage />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let supported_languages = LANGUAGES.to_vec();
    let translations = translations();

    html! {
        <I18nProvider supported_languages={supported_languages} translations={translations} >
            <BrowserRouter>
                <Sidenav />
                <Navigation />
                <Switch<Route> render={switch_main} />
            </BrowserRouter>
        </I18nProvider>
    }
}
