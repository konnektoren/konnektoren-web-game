use konnektoren_core::challenges::{ChallengeType, MultipleChoice};
use konnektoren_core::game::Game;
use konnektoren_yew::components::challenge::multiple_choice::MultipleChoiceComponentProps;
use konnektoren_yew::components::challenge::{
    MultipleChoiceCircleComponent, MultipleChoiceComponent,
};
use konnektoren_yew::prelude::challenge::SortTableComponent;
use konnektoren_yew::prelude::*;
use yew::prelude::*;
use yew_preview::create_component_item;
use yew_preview::prelude::*;

#[function_component(PreviewPage)]
pub fn preview_page() -> Html {
    let game = Game::default();
    let default_challenge = game.create_challenge("konnektoren-1").unwrap();
    let default_multiple_choice: MultipleChoice = match &default_challenge.challenge_type {
        ChallengeType::MultipleChoice(multiple_choice) => multiple_choice.clone(),
        _ => unreachable!(),
    };

    let components: ComponentList = vec![
        BlinkAnimation::preview(),
        create_component_item!(
            "MultipleChoiceComponent",
            MultipleChoiceComponent,
            vec![(
                "default",
                MultipleChoiceComponentProps {
                    challenge: default_multiple_choice.clone(),
                    ..Default::default()
                }
            )]
        ),
        create_component_item!(
            "MultipleChoiceCircleComponent",
            MultipleChoiceCircleComponent,
            vec![(
                "default",
                MultipleChoiceComponentProps {
                    challenge: default_multiple_choice.clone(),
                    ..Default::default()
                }
            )]
        ),
        SortTableComponent::preview(),
        ChallengeComponent::preview(),
        ProfileConfigComponent::preview(),
        ProfilePointsComponent::preview(),
        ChallengeActionsComponent::preview(),
        ChallengeConfigComponent::preview(),
        ChallengeInfoComponent::preview(),
        GameMapComponent::preview(),
        GamePathComponent::preview(),
        MusicComponent::preview(),
        OptionsComponent::preview(),
        ProgressBar::preview(),
        QuestionComponent::preview(),
        TranslateComponent::preview(),
    ];

    html! {
        <div>
            <yew_preview::prelude::PreviewPage {components} />
        </div>
    }
}
