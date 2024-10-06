use crate::components::{Chat, VibrateEffectComponent};
use crate::utils::translation::i18n_macro::selected_language;
use gloo::timers::callback::Timeout;
use konnektoren_core::challenges::ChallengeVariant;
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use konnektoren_yew::components::challenge::ChallengeEvent;
use konnektoren_yew::components::{ChallengeComponent, MusicComponent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge: Challenge,
    #[prop_or_default]
    pub variant: Option<ChallengeVariant>,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
}

#[function_component(ChallengeEffectComponent)]
pub fn challenge_effect_component(props: &Props) -> Html {
    let challenge = props.challenge.clone();
    let effect_ref = use_state(|| html! { <div></div> });
    let counter = use_state(|| 0);
    let language = selected_language();

    let handle_finish = {
        let on_finish = props.on_finish.clone();
        Callback::from(move |result: ChallengeResult| {
            if let Some(on_finish) = on_finish.as_ref() {
                on_finish.emit(result.clone());
            }
        })
    };

    let handle_event = {
        let on_finish = props.on_finish.clone();
        let effect_ref = effect_ref.clone();
        let counter = counter.clone();
        Callback::from(move |event: ChallengeEvent| match event {
            ChallengeEvent::Finish(result) => {
                if let Some(on_finish) = on_finish.as_ref() {
                    on_finish.emit(result.clone());
                }
            }
            ChallengeEvent::NextTask(_index) => {}
            ChallengeEvent::PreviousTask(_index) => {}
            ChallengeEvent::SolvedCorrect(_index) => {
                let c = *counter + 1;
                counter.set(c);
                let effect = html! {
                    <>
                        <div class="challenge-effect__blink challenge-effect__blink--correct"></div>
                        <MusicComponent id={format!("challenge-effect__music--correct-{}", c)} url="/music/UI Positive Signal 002.wav" repeat={false} />
                    </>
                };
                effect_ref.set(effect);
                let effect_ref_clone = effect_ref.clone();
                Timeout::new(2000, move || {
                    effect_ref_clone.set(html! { <div></div> });
                })
                .forget();
            }
            ChallengeEvent::SolvedIncorrect(_index) => {
                let c = *counter + 1;
                counter.set(c);
                let effect = html! {
                    <>
                        <div class="challenge-effect__blink challenge-effect__blink--incorrect"></div>
                        <MusicComponent id={format!("challenge-effect__music--incorrect-{}", c)} url="/music/UI Negative Signal 003.wav" repeat={false} />
                        <VibrateEffectComponent duration={100} key={c} />
                    </>
                };
                effect_ref.set(effect);
                let effect_ref_clone = effect_ref.clone();
                Timeout::new(2000, move || {
                    effect_ref_clone.set(html! { <div></div> });
                })
                .forget();
            }
        })
    };

    html! {
        <div class="challenge-effect" id={format!("challenge-effect-{}", *counter)}>
            {(*effect_ref).clone()}
            <div class="challenge-effect__content">
                <ChallengeComponent
                    challenge={challenge.clone()}
                    variant={props.variant.clone()}
                    on_finish={handle_finish}
                    on_event={handle_event}
                    {language}
                />
                <div class="challenge-effect__chat">
                    <Chat challenge_id={challenge.challenge_config.id.clone()} />
                </div>
            </div>
        </div>
    }
}
