use crate::i18n;
use konnektoren_yew::components::profile::ProfileConfigComponent;
use yew::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    html! {
        <div class="profile-page">
            <h1>{ i18n!("Your Profile") }</h1>
            <ProfileConfigComponent />
        </div>
    }
}
