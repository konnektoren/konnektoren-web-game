use gloo::timers::callback::Timeout;
use konnektoren_yew::components::ChallengeReviewComponent;
use konnektoren_yew::i18n::use_i18n;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

const FEEDBACK_START_SECONDS: u32 = 30;
const FEEDBACK_END_SECONDS: u32 = 120;

#[function_component(FeedbackPopup)]
pub fn feedback_popup() -> Html {
    let show = use_state(|| false);
    let expanded = use_state(|| false);
    let timeout_handle = use_state(|| Rc::new(RefCell::new(None)));

    let i18n = use_i18n();

    {
        let show = show.clone();
        let timeout_handle = timeout_handle.clone();
        use_effect_with((), move |_| {
            let mut rng = rand::thread_rng();
            let delay = rng.gen_range(FEEDBACK_START_SECONDS..FEEDBACK_END_SECONDS);

            let handle = Timeout::new(delay * 1000, move || {
                show.set(true);
            });

            timeout_handle.set(Rc::new(RefCell::new(Some(handle))));

            move || {
                if let Some(handle) = timeout_handle.borrow_mut().take() {
                    handle.cancel();
                }
            }
        });
    }

    let on_close = {
        let show = show.clone();
        Callback::from(move |_| {
            show.set(false);
        })
    };

    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    if !*show {
        return html! {};
    }

    html! {
        <div class={classes!("feedback-popup", if *expanded { "expanded" } else { "" })}>
            <div class="feedback-bubble" onclick={on_toggle.clone()}>
                <span class="feedback-icon">{"💬"}</span>
                <span class="feedback-text">{i18n.t("Feedback")}</span>
            </div>
            if *expanded {
                <div class="feedback-content">
                    <button class="close-button" onclick={on_close}>{"×"}</button>
                        <h3>{i18n.t("We'd love your feedback!")}</h3>
                    <ChallengeReviewComponent
                        api_url="https://api.konnektoren.help/api/v1/reviews"
                        challenge_id="general_feedback"
                    />
                </div>
            }
        </div>
    }
}
