use gloo::utils::window;
use yew::prelude::*;

const CLIENT_ID: &str = env!("GOOGLE_CLIENT_ID");
const REDIRECT_URI: &str = env!("GOOGLE_REDIRECT_URI");

#[derive(Properties, PartialEq)]
pub struct GoogleOAuthProps {
    pub on_token: Callback<String>,
}

#[function_component(GoogleOAuthComponent)]
pub fn google_oauth(props: &GoogleOAuthProps) -> Html {
    let handle_login = {
        Callback::from(move |_| {
            let scope = "https://www.googleapis.com/auth/drive.file";
            let auth_url = format!(
                "https://accounts.google.com/o/oauth2/v2/auth?\
                client_id={}&\
                redirect_uri={}&\
                response_type=token&\
                scope={}&\
                prompt=consent", // Added prompt=consent instead of access_type=offline
                CLIENT_ID, REDIRECT_URI, scope
            );
            window().location().set_href(&auth_url).unwrap();
        })
    };

    html! {
        <div class="google-oauth">
            <button
                onclick={handle_login}
                class="google-oauth__button"
            >
                <img
                    src="https://developers.google.com/identity/images/g-logo.png"
                    alt="Google logo"
                    class="google-oauth__icon"
                />
                {"Sign in with Google"}
            </button>
        </div>
    }
}
