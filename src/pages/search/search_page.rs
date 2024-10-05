use crate::components::map::ChallengeInfo;
use crate::pages::search::search_challenges::SearchChallenges;
use web_sys::{HtmlInputElement, UrlSearchParams};
use yew::prelude::*;
use konnektoren_yew::i18n::use_i18n;

#[derive(Properties, Clone, PartialEq)]
pub struct SearchPageProps {
    #[prop_or_default]
    pub search_query: Option<String>,
}

#[function_component(SearchPage)]
pub fn search_page(props: &SearchPageProps) -> Html {
    let i18n = use_i18n();
    let query = web_sys::window().unwrap().location().search().unwrap();

    let search_query = if let Some(search_query) = &props.search_query {
        search_query.clone()
    } else {
        let search_params = UrlSearchParams::new_with_str(&query).unwrap();
        search_params.get("query").unwrap_or_default()
    };

    let search_query = use_state(|| search_query);

    let suggestion = use_state(String::new);
    let search_engine = use_state(SearchChallenges::new);

    let on_search_query_change = {
        let search_query = search_query.clone();
        let suggestion = suggestion.clone();
        let search_engine = search_engine.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            search_query.set(value.clone());

            if let Some(sugg) = search_engine.get_suggestion(&value) {
                suggestion.set(sugg);
            } else {
                suggestion.set(String::new());
            }
        })
    };

    let on_suggestion_select = {
        let search_query = search_query.clone();
        let suggestion = suggestion.clone();
        Callback::from(move |_| {
            search_query.set((*suggestion).clone());
            suggestion.set(String::new());
        })
    };

    let challenges = search_engine.search(&search_query);

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

    let suggestion = (*suggestion).clone();

    html! {
        <div class="search-page">
            <h1>{"Search Challenges"}</h1>
            <div class="search-container">
                <input
                    id="search-query"
                    type="text"
                    value={(*search_query).clone()}
                    oninput={on_search_query_change}
                    placeholder= {i18n.t("Type to search...")}
                />
                if !suggestion.is_empty() && suggestion != *search_query {
                    <div class="suggestion" onclick={on_suggestion_select}>
                        {format!("Did you mean: {}", suggestion)}
                    </div>
                }
            </div>
            <div class="search-results">
                <p>{format!("{} {} {}", i18n.t("Found"), challenges.len(), i18n.t("challenges"))}</p>
                {challenges_list}
            </div>
        </div>
    }
}
