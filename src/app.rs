use crate::{
    pages::about::About,
    utils::translation::{languages, translations},
};
use yew::prelude::*;
use yew_i18n::I18nProvider;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let supported_languages = languages();
        let translations = translations();
        html! {
            <I18nProvider supported_languages={supported_languages} translations={translations} >

            <div>
                <h1>{ "Coming soon!" }</h1>
                <About />
            </div>
            </I18nProvider>
        }
    }
}
