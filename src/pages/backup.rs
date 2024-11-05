use gloo::utils::{document, window};
use konnektoren_core::session::Session;
use konnektoren_yew::prelude::{ChallengeHistorySummaryComponent, GDriveBackupComponent};
use konnektoren_yew::providers::{use_session, use_session_repository};
use konnektoren_yew::repository::SESSION_STORAGE_KEY;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::UrlSearchParams;
use web_sys::{Blob, Event, HtmlInputElement};
use yew::prelude::*;

const GOOGLE_CLIENT_ID: &str = env!("GOOGLE_CLIENT_ID");
const REDIRECT_URI: &str = env!("GOOGLE_REDIRECT_URI");

#[function_component(BackupPage)]
pub fn backup_page() -> Html {
    let session = use_session();
    let session_repository = use_session_repository();
    let selected_session = use_state(|| None::<Session>);
    let show_success = use_state(|| false);

    let google_access_token = use_state(|| None::<String>);

    {
        let google_access_token = google_access_token.clone();
        use_effect_with((), move |_| {
            let location = window().location();

            if let Ok(hash) = location.hash() {
                log::info!("Got hash: {}", hash);

                if !hash.is_empty() {
                    // Remove the leading '#'
                    if let Ok(params) = UrlSearchParams::new_with_str(&hash[1..]) {
                        if let Some(token) = params.get("access_token") {
                            log::info!("Successfully extracted access token");
                            google_access_token.set(Some(token));

                            // Clear the URL hash
                            if let Ok(history) = window().history() {
                                let _ = history.replace_state_with_url(
                                    &JsValue::NULL,
                                    "",
                                    Some("/backup"),
                                );
                            }
                        } else {
                            log::warn!("No access_token found in hash params");
                        }

                        // You might want to store these additional values
                        if let Some(expires_in) = params.get("expires_in") {
                            log::info!("Token expires in {} seconds", expires_in);
                        }
                        if let Some(scope) = params.get("scope") {
                            log::info!("Token scope: {}", scope);
                        }
                    } else {
                        log::error!("Failed to parse hash params");
                    }
                }
            }
        });
    }

    // Set page title
    use_effect(move || {
        document().set_title("Konnektoren - Backup");
        || ()
    });

    let on_download = {
        let session = session.clone();
        Callback::from(move |_| {
            let session = session.clone();
            let session: Session = (&*session).clone();

            log::info!(
                "Downloading Session: {:?}",
                session.game_state.game.challenge_history.len()
            );

            let session_data = serde_json::to_string(&session).unwrap();

            let json_jsvalue = JsValue::from_str(&session_data);
            let json_jsvalue_array = js_sys::Array::from_iter(std::iter::once(json_jsvalue));

            let json_blob_result = Blob::new_with_str_sequence(&json_jsvalue_array);
            let blob = json_blob_result.unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

            let document = gloo::utils::document();
            let anchor = document.create_element("a").unwrap();
            anchor.set_attribute("href", &url).unwrap();
            anchor
                .set_attribute("download", "konnektoren-backup.json")
                .unwrap();

            let anchor = anchor.dyn_into::<web_sys::HtmlElement>().unwrap();
            anchor.click();
        })
    };

    let on_file_change = {
        let selected_file_content = selected_session.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    // Create a reader
                    let reader = web_sys::FileReader::new().unwrap();
                    let selected_content = selected_file_content.clone();
                    let reader_clone = reader.clone();

                    // Create onload callback
                    let onload = Closure::wrap(Box::new(move |_e: Event| {
                        let text = reader_clone.result().unwrap().as_string().unwrap();

                        match serde_json::from_str::<Session>(&text) {
                            Ok(loaded_session) => {
                                log::debug!(
                                    "Successfully parsed session, {}",
                                    loaded_session.game_state.game.challenge_history.len()
                                );
                                selected_content.set(Some(loaded_session));
                            }
                            Err(e) => {
                                log::error!("Failed to parse backup file: {:?}", e);
                                selected_content.set(None);
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    // Set onload handler and read the file
                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    reader.read_as_text(&file).unwrap();

                    // Prevent closure from being dropped
                    onload.forget();
                } else {
                    log::error!("No file selected");
                }
            } else {
                log::error!("No files in input");
            }
        })
    };

    let on_load = {
        let session = session.clone();
        let session_repository = session_repository.clone();
        let selected_file_content = selected_session.clone();
        let show_success = show_success.clone();
        Callback::from(move |_| {
            if let Some(loaded_session) = (*selected_file_content).clone() {
                let session = session.clone();
                let session_repository = session_repository.clone();
                let selected_file_content = selected_file_content.clone();
                let show_success = show_success.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(()) = session_repository
                        .save_session(SESSION_STORAGE_KEY, &loaded_session)
                        .await
                    {
                        session.set(loaded_session);
                        selected_file_content.set(None);
                        show_success.set(true);

                        let show_success = show_success.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo::timers::future::sleep(std::time::Duration::from_secs(3)).await;
                            show_success.set(false);
                        });
                    }
                });
            }
        })
    };

    let challenge_history_component = {
        if let Some(session) = &*selected_session {
            let challenge_history = session.game_state.game.challenge_history.clone();
            log::info!("Challenge history: {:?}", challenge_history.len());
            html! {
                <ChallengeHistorySummaryComponent {challenge_history} />
            }
        } else {
            html! {}
        }
    };

    let on_drive_session_selected = {
        let selected_session = selected_session.clone();
        Callback::from(move |session: Session| {
            selected_session.set(Some(session));
        })
    };

    html! {
        <div class="backup-page">
            <h1 class="backup-page__title">{"Backup and Restore"}</h1>

            if let Some(access_token) = google_access_token.as_ref() {
                <div class="backup-page__section">
                    <h2 class="backup-page__section-title">{"Saved Sessions in Google Drive"}</h2>
                    <div class="backup-page__section-content">
                        <GDriveBackupComponent
                            access_token={Some(access_token.clone())}
                            client_id={GOOGLE_CLIENT_ID.to_string()}
                            redirect_uri={REDIRECT_URI.to_string()}
                            session={(*session).clone()}
                            on_select={on_drive_session_selected}
                        />
                    </div>
                </div>
            } else {
                <div class="backup-page__section">
                    <h2 class="backup-page__section-title">{"Connect to Google Drive"}</h2>
                    <div class="backup-page__section-content">
                    <GDriveBackupComponent
                        access_token={None::<String>}
                        client_id={GOOGLE_CLIENT_ID.to_string()}
                        redirect_uri={REDIRECT_URI.to_string()}
                        session={(*session).clone()}
                        on_select={on_drive_session_selected}
                    />
                    </div>
                </div>
                <div class="backup-page__section backup-page__section--download">
                    <h2 class="backup-page__section-title">{"Backup Current Session"}</h2>
                    <div class="backup-page__section-content">
                        <button class="backup-page__download-button" onclick={on_download}>
                            {"Download Backup"}
                        </button>
                    </div>
                </div>
                <div class="backup-page__section backup-page__section--upload">
                    <h2 class="backup-page__section-title">{"Restore Session"}</h2>
                    <div class="backup-page__section-content">
                        <input
                            type="file"
                            accept=".json"
                            class="backup-page__upload-input"
                            onchange={on_file_change}
                        />
                        <p class="backup-page__help-text">
                            {"Select a backup file to restore your previous session"}
                        </p>
                    </div>
                </div>
            }

            if *show_success {
                <div class="backup-page__message backup-page__message--success">
                    {"Session loaded successfully!"}
                </div>
            }

            if (*selected_session).is_some() {
                <div class="backup-page__section">
                    <h2 class="backup-page__section-title">{"Selected Backup Content"}</h2>

                    <button
                            class="backup-page__load-button"
                            onclick={on_load}
                        >
                            {"Load Selected Backup"}
                    </button>

                    <div class="backup-page__section-content">
                        {challenge_history_component}
                    </div>
                </div>
            }
        </div>
    }
}
