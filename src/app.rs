use crate::{
    components::Navigation,
    pages::{AboutPage, ChallengePage, HomePage, MapPage, ProfilePage},
    route::Route,
    utils::translation::{translations, LANGUAGES},
};
use yew::prelude::*;
use yew_i18n::I18nProvider;

use crate::components::Sidenav;
use yew_router::{BrowserRouter, Switch};

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<AboutPage /> },
        Route::Challenge { id } => html! {<ChallengePage {id} />},
        Route::Home => html! {<HomePage />},
        Route::Map => html! {<MapPage />},
        Route::Profile => html! {<ProfilePage />},
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
}
