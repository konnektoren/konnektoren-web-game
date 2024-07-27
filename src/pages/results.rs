use gloo::utils::{document, window};
use konnektoren_core::certificates::CertificateData;
use konnektoren_yew::prelude::CertificateComponent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultsProps {
    pub code: Option<String>,
    #[prop_or_default]
    pub hostname: Option<String>,
    #[prop_or_default]
    pub protocol: Option<String>,
}

#[function_component(ResultsPage)]
pub fn results(props: &ResultsProps) -> Html {
    use_effect(|| {
        document().set_title(&format!("Konnektoren - {}", "Certificate"));
        || ()
    });
    let code: String = props.code.as_ref().unwrap_or(&"".to_string()).clone();

    let hostname = props
        .hostname
        .clone()
        .unwrap_or(window().location().host().unwrap_or_default());
    let protocol = props
        .protocol
        .clone()
        .unwrap_or(window().location().protocol().unwrap_or_default());

    let certificate_data = CertificateData::from_base64(&code);

    match certificate_data {
        Ok(certificate_data) => {
            html! {
                <div class="results-page">
                    <CertificateComponent certificate_data={certificate_data} hostname={hostname} protocol={protocol} />
                </div>
            }
        }
        Err(error_message) => html! {
            <div class="results-page">
                <h1>{ "Oops!" }</h1>
                <p>{ "It seems like something went wrong with displaying your certificate." }</p>
                <p>{ error_message.to_string() }</p>
                <p>{ "Maybe the code was incorrect, or perhaps our server just needs a coffee break. 🤷‍♂️ ☕" }</p>
            </div>
        },
    }
}
