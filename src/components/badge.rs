use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub label: String,
    pub description: String,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    html! {
        <div class="badge-container">
            <span class="badge">{ &props.label }</span>
            <div class="badge-description">
                { &props.description }
            </div>
        </div>
    }
}
