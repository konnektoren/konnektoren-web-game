use konnektoren_yew::prelude::use_i18n;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SessionPageProps {
    pub id: String,
}

#[function_component(SessionPage)]
pub fn session_page(props: &SessionPageProps) -> Html {
    let i18n = use_i18n();

    html! {
        <div class="session-page">
            <h1>{ i18n.t("Session") }</h1>
        </div>
    }
}
