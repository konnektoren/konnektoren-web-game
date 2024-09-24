use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Roulette)]
pub fn roulette() -> Html {
    let links = vec![
        ("articles-1", "der die das"),
        ("reflexivpronoun-1", "Reflexive Pronomen"),
        ("personal_pronouns-1", "Personal Pronomen"),
        ("konnektoren-1", "Konnektoren"),
    ];

    html! {
        <div class="roulette">
            { for links.iter().enumerate().map(|(i, (id, text))| html! {
                <div class={format!("slice slice-{}", i)}>
                    <Link<Route> to={Route::Challenge { id: id.to_string() }}>
                        <span class="link-text">{ text }</span>
                    </Link<Route>>
                </div>
            }) }
            <div class="center-circle">
                <Link<Route> to={Route::Challenges}>
                    <span class="link-text">{ "Exercises" }</span>
                </Link<Route>>
            </div>
        </div>
    }
}
