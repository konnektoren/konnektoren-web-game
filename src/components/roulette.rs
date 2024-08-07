use yew::prelude::*;
use yew_router::prelude::Link;
use crate::route::Route;

#[function_component(Roulette)]
pub fn roulette() -> Html {

    html! {
        <div class="roulette">
        <ul>
          <li>  <Link<Route> to={Route::Challenge {
                id: "articles-1".to_string()
            }}> <span class="link-text">{ "der die das" }</span>
            </Link<Route>>
          </li>
          <li>   <Link<Route> to={Route::Challenge {
                id: "reflexivpronoun-1".to_string()
            }}> <span class="link-text">{ "Reflexive Pronomen" }</span>
            </Link<Route>>
          </li>
          <li>   <Link<Route> to={Route::Challenge {
                id: "personal_pronouns-1".to_string()
            }}> <span class="link-text">{ "Personal Pronomen" }</span>
            </Link<Route>>
          </li>
        </ul>
        </div>
    }
}
