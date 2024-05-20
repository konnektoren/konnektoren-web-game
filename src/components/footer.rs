use crate::i18n;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="container text-center">
                <p>{ i18n!("The Konnektoren examples featured here are derived from the German DTB C1 Course.")}</p>
                <p>{ i18n!("Special thanks to the educators and learners at the IFS Academy for their contributions to the course materials.")}</p>
            </div>
        </footer>
    }
}
