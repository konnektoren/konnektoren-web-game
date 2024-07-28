use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let i18n = use_i18n();
    html! {
        <footer class="footer">
            <div class="container text-center">
                <p>{ i18n.t("The Konnektoren examples featured here are derived from the German DTB C1 Course.")}</p>
                <p>{ i18n.t("Special thanks to the educators and learners at the IFS Academy for their contributions to the course materials.")}</p>
            </div>
        </footer>
    }
}
