use gloo::utils::document;
use konnektoren_core::session::Session;
use konnektoren_yew::prelude::ChallengeHistorySummaryComponent;
use konnektoren_yew::providers::{use_session, use_session_repository};
use konnektoren_yew::repository::SESSION_STORAGE_KEY;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, Event, HtmlInputElement};
use yew::prelude::*;

#[function_component(BackupPage)]
pub fn backup_page() -> Html {
    let session = use_session();
    let session_repository = use_session_repository();
    let selected_file_content = use_state(|| None::<Session>);
    let show_success = use_state(|| false);

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
        let selected_file_content = selected_file_content.clone();
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
        let selected_file_content = selected_file_content.clone();
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
        if let Some(session) = &*selected_file_content {
            let challenge_history = session.game_state.game.challenge_history.clone();
            log::info!("Challenge history: {:?}", challenge_history.len());
            html! {
                <ChallengeHistorySummaryComponent {challenge_history} />
            }
        } else {
            html! {}
        }
    };

    html! {
        <div class="backup-page">
            <h1 class="backup-page__title">{"Backup and Restore"}</h1>

            if *show_success {
                <div class="backup-page__message backup-page__message--success">
                    {"Session loaded successfully!"}
                </div>
            }

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

                    if (*selected_file_content).is_some() {
                        <button
                            class="backup-page__load-button"
                            onclick={on_load}
                        >
                            {"Load Selected Backup"}
                        </button>
                    }
                </div>
            </div>

            if (*selected_file_content).is_some() {
                <div class="backup-page__section">
                    <h2 class="backup-page__section-title">{"Selected Backup Content"}</h2>
                    <div class="backup-page__section-content">
                        {challenge_history_component}
                    </div>
                </div>
            }
        </div>
    }
}
