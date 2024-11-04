use gloo::net::http::Request;
use konnektoren_core::session::Session;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GoogleDriveFile {
    id: String,
    name: String,
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

    html! {
        <div class="google-drive-sessions-component">
            if let Some(err) = &*error {
                <div class="error-message">
                    {format!("Error: {}", match err {
                        LoadError::NetworkError(msg) => format!("Network error: {}", msg),
                        LoadError::ParseError(msg) => format!("Failed to parse response: {}", msg),
                        LoadError::ApiError(msg) => format!("API error: {}", msg),
                    })}
                </div>
            }

            if *loading {
                <div class="loading">{"Loading saved sessions..."}</div>
            } else {
                <ul class="google-drive-sessions">
                    {
                        (*sessions).clone().into_iter().map(|session| {
                            let handle_select = handle_session_select.clone();
                            let id = session.id.clone();
                            html! {
                                <li
                                    onclick={move |_| handle_select.emit(id.clone())}
                                    key={id.clone()}
                                >
                                    {&session.name}
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            }
        </div>
    }
}

async fn fetch_google_drive_files(access_token: &str) -> Result<Vec<GoogleDriveFile>, LoadError> {
    let url = "https://www.googleapis.com/drive/v3/files";
    let response = Request::get(url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
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

    let response: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| LoadError::ParseError(e.to_string()))?;

    let files = response
        .as_object()
        .and_then(|obj| obj.get("files"))
        .and_then(|files| files.as_array())
        .ok_or_else(|| LoadError::ParseError("Missing 'files' array in response".to_string()))?
        .iter()
        .map(|file| {
            let id = file["id"]
                .as_str()
                .ok_or_else(|| LoadError::ParseError("Missing 'id' in file".to_string()))?
                .to_string();
            let name = file["name"]
                .as_str()
                .ok_or_else(|| LoadError::ParseError("Missing 'name' in file".to_string()))?
                .to_string();
            Ok(GoogleDriveFile { id, name })
        })
        .collect::<Result<Vec<_>, LoadError>>()?;

    Ok(files)
}

async fn fetch_session_from_drive(access_token: &str, file_id: &str) -> Result<Session, LoadError> {
    let url = format!(
        "https://www.googleapis.com/drive/v3/files/{}/export?alt=media",
        file_id
    );
    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
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

    serde_json::from_str::<Session>(&text)
        .map_err(|e| LoadError::ParseError(format!("Failed to parse session: {}", e)))
}
