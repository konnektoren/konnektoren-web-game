use gloo::net::http::Request;
use konnektoren_core::session::Session;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GoogleDriveFile {
    id: String,
    name: String,
    #[serde(rename = "createdTime")]
    created_time: Option<String>,
}

#[derive(Debug)]
enum LoadError {
    NetworkError(String),
    ParseError(String),
    ApiError(String),
}

#[derive(Properties, PartialEq)]
pub struct GoogleDriveSessionsProps {
    pub access_token: String,
    pub on_session_selected: Callback<Session>,
    pub on_session_uploaded: Callback<()>,
    pub session: Session,
}

#[function_component(GoogleDriveSessionsComponent)]
pub fn google_drive_sessions(props: &GoogleDriveSessionsProps) -> Html {
    let sessions = use_state(|| Vec::<GoogleDriveFile>::new());
    let loading = use_state(|| false);
    let error = use_state(|| None::<LoadError>);

    {
        let sessions = sessions.clone();
        let loading = loading.clone();
        let error = error.clone();
        let access_token = props.access_token.clone();
        use_effect_with((), move |_| {
            loading.set(true);
            error.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_google_drive_files(&access_token).await {
                    Ok(files) => {
                        sessions.set(files);
                        loading.set(false);
                    }
                    Err(err) => {
                        error.set(Some(err));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    let handle_session_select = {
        let on_session_selected = props.on_session_selected.clone();
        let access_token = props.access_token.clone();
        let error = error.clone();
        Callback::from(move |id: String| {
            let on_session_selected = on_session_selected.clone();
            let access_token = access_token.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_session_from_drive(&access_token, &id).await {
                    Ok(session) => {
                        on_session_selected.emit(session);
                        error.set(None);
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                }
            });
        })
    };

    let handle_session_upload = {
        let access_token = props.access_token.clone();
        let on_session_uploaded = props.on_session_uploaded.clone();
        let error = error.clone();
        let session = props.session.clone();
        let sessions = sessions.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let access_token = access_token.clone();
            let on_session_uploaded = on_session_uploaded.clone();
            let error = error.clone();
            let session = session.clone();
            let sessions = sessions.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match upload_session(&access_token, &session).await {
                    Ok(_) => {
                        on_session_uploaded.emit(());
                        error.set(None);

                        loading.set(true);
                        match fetch_google_drive_files(&access_token).await {
                            Ok(files) => {
                                sessions.set(files);
                                loading.set(false);
                            }
                            Err(err) => {
                                error.set(Some(err));
                                loading.set(false);
                            }
                        }
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                }
            });
        })
    };
    html! {
        <div class="google-drive-sessions">
            if let Some(err) = &*error {
                <div class="google-drive-sessions__error">
                    <i class="fas fa-exclamation-circle google-drive-sessions__error-icon"></i>
                    {format!("Error: {}", match err {
                        LoadError::NetworkError(msg) => format!("Network error: {}", msg),
                        LoadError::ParseError(msg) => format!("Failed to parse response: {}", msg),
                        LoadError::ApiError(msg) => format!("API error: {}", msg),
                    })}
                </div>
            }

            if *loading {
                <div class="google-drive-sessions__loading">
                    <i class="fas fa-spinner fa-spin google-drive-sessions__loading-icon"></i>
                    <span class="google-drive-sessions__loading-text">{"Loading saved sessions..."}</span>
                </div>
            } else {
                <div class="google-drive-sessions__content">
                    <button
                        class="google-drive-sessions__button"
                        onclick={handle_session_upload}
                    >
                        <i class="fas fa-cloud-upload-alt google-drive-sessions__button-icon"></i>
                        <span class="google-drive-sessions__button-text">{"Upload Session"}</span>
                    </button>
                    <ul class="google-drive-sessions__list">
                        {
                            (*sessions).clone().into_iter().map(|session| {
                                let handle_select = handle_session_select.clone();
                                let id = session.id.clone();
                                html! {
                                    <li
                                        class="google-drive-sessions__item"
                                        onclick={move |_| handle_select.emit(id.clone())}
                                        key={id.clone()}
                                    >
                                        <i class="fas fa-file-alt google-drive-sessions__item-icon"></i>
                                        <div class="google-drive-sessions__item-content">
                                            <span class="google-drive-sessions__item-name">{&session.name}</span>
                                            if let Some(created_time) = &session.created_time {
                                                if let Ok(dt) = DateTime::parse_from_rfc3339(created_time) {
                                                    <span class="google-drive-sessions__item-date">
                                                        <i class="far fa-clock google-drive-sessions__item-date-icon"></i>
                                                        {dt.format("%Y-%m-%d %H:%M").to_string()}
                                                    </span>
                                                }
                                            }
                                        </div>
                                    </li>
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                </div>
            }
        </div>
    }
}

async fn fetch_google_drive_files(access_token: &str) -> Result<Vec<GoogleDriveFile>, LoadError> {
    let url = "https://www.googleapis.com/drive/v3/files?\
               spaces=drive&\
               fields=files(id,name,createdTime)&\
               orderBy=createdTime desc&\
               q=mimeType='application/json' and name contains 'konnektoren-backup'";

    let response = Request::get(url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| LoadError::NetworkError(e.to_string()))?;

    if !response.ok() {
        return Err(LoadError::ApiError(format!(
            "API returned status: {}",
            response.status()
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| LoadError::NetworkError(e.to_string()))?;

    log::info!("Google Drive API response: {}", text);

    let response: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| LoadError::ParseError(e.to_string()))?;

    let files = response
        .get("files")
        .and_then(|files| files.as_array())
        .ok_or_else(|| LoadError::ParseError("Missing 'files' array in response".to_string()))?
        .iter()
        .filter_map(|file| {
            let id = file.get("id")?.as_str()?.to_string();
            let name = file.get("name")?.as_str()?.to_string();
            let created_time = file
                .get("createdTime")
                .and_then(|t| t.as_str())
                .map(String::from);
            Some(GoogleDriveFile {
                id,
                name,
                created_time,
            })
        })
        .collect();

    Ok(files)
}

async fn fetch_session_from_drive(access_token: &str, file_id: &str) -> Result<Session, LoadError> {
    // Use the direct download endpoint instead of export
    let url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?alt=media",
        file_id
    );

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| LoadError::NetworkError(e.to_string()))?;

    if !response.ok() {
        return Err(LoadError::ApiError(format!(
            "API returned status: {}",
            response.status()
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| LoadError::NetworkError(e.to_string()))?;

    log::info!("File content response: {}", text);

    serde_json::from_str::<Session>(&text)
        .map_err(|e| LoadError::ParseError(format!("Failed to parse session: {}", e)))
}

use chrono::{DateTime, Utc};

async fn upload_session(access_token: &str, session: &Session) -> Result<(), LoadError> {
    // Get current UTC time and format it
    let now: DateTime<Utc> = Utc::now();
    let date_string = now.format("%Y-%m-%d_%H-%M-%S").to_string();

    let filename = format!("konnektoren-backup_{}.json", date_string);

    let session_data = serde_json::to_string(session).unwrap();
    let metadata = serde_json::json!({
        "name": filename,
        "mimeType": "application/json"
    });

    let boundary = "foo_bar_baz";
    let body = format!(
        "--{}\r\n\
        Content-Type: application/json; charset=UTF-8\r\n\r\n\
        {}\r\n\
        --{}\r\n\
        Content-Type: application/json\r\n\r\n\
        {}\r\n\
        --{}--",
        boundary,
        metadata.to_string(),
        boundary,
        session_data,
        boundary
    );

    let upload_url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";
    let request = Request::post(upload_url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Content-Type", "multipart/related; boundary=foo_bar_baz")
        .body(body)
        .unwrap();

    let response = request
        .send()
        .await
        .map_err(|e| LoadError::NetworkError(e.to_string()))?;

    if !response.ok() {
        return Err(LoadError::ApiError(format!(
            "API returned status: {}",
            response.status()
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| LoadError::NetworkError(e.to_string()))?;

    log::info!("Upload response: {:?}", text);

    Ok(())
}
