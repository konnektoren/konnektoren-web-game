use crate::components::map::ChallengeInfo;
use crate::pages::search::search_challenges::SearchChallenges;
use web_sys::UrlSearchParams;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SearchPageProps {
    #[prop_or_default]
    pub search_query: Option<String>,
}

#[function_component(SearchPage)]
pub fn search_page(props: &SearchPageProps) -> Html {
    let query = web_sys::window().unwrap().location().search().unwrap();
    let search_query = use_state(|| {
        props.search_query.clone().or_else(|| {
            let search_params = UrlSearchParams::new_with_str(&query).unwrap();
            search_params.get("q")
        })
    });
    let search_engine = use_state(|| SearchChallenges::new());

    let on_search_query_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            search_query.set(Some(input));
        })
    };

    let challenges = match (*search_query).clone() {
        Some(query) if !query.is_empty() => (*search_engine).search(&query),
        Some(_) => vec![],
        None => vec![],
    };

    let challenges_list = challenges
        .iter()
        .enumerate()
        .map(|(index, config)| {
            html! {
                <div class="challenge" key={index}>
                    <ChallengeInfo challenge_config={config.clone()} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="search-page">
            <h1>{"Search Page"}</h1>
            <label for="search-query">{"Search"}</label>
            <input
                id="search-query"
                type="text"
                value={(*search_query).clone().unwrap_or_default()}
                oninput={on_search_query_change}
            />
            <div class="search-results">
                {challenges_list}
            </div>
        </div>
    }
}
