use crate::model::WebSession;
use gloo::net::http::Request;
use konnektoren_core::certificates::CertificateData;
use konnektoren_core::challenges::PerformanceRecord;
use konnektoren_yew::storage::{ProfileStorage, Storage};
use qrcodegen::{QrCode, QrCodeEcc};
use wasm_bindgen_futures;
use yew::prelude::*;

const ISSUER_URL: &str = "https://vc.konnektoren.help";

#[function_component(VerifiableCredentialComponent)]
pub fn verifiable_credential_component() -> Html {
    let offer_state = use_state(|| None::<String>);

    let on_claim_offer = {
        let offer_state = offer_state.clone();
        Callback::from(move |_| {
            let offer_state = offer_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Claim offer button clicked");
                let certificate = generate_performance_record();

                match fetch_offer(&certificate).await {
                    Ok(offer_url) => offer_state.set(Some(offer_url)),
                    Err(e) => log::error!("Error fetching offer: {:?}", e),
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "Verifiable Credential" }</h2>
            <button onclick={on_claim_offer}>{ "Generate Claim Offer" }</button>
            {
                if let Some(offer_url) = (*offer_state).clone() {
                    render_offer(offer_url)
                } else {
                    html! {}
                }
            }
            <p>{ "Download Identity Wallet to claim your offer" }</p>
            <a href="https://play.google.com/store/apps/details?id=com.impierce.identity_wallet&pcampaignid=web_share">
                <img src="https://raw.githubusercontent.com/impierce/identity-wallet/b110b9670fbdf3c69a18d12be72ed91e3eded7ef/.github/banner.svg" alt="Get Identity Wallet on Google Play"
               width="200px" />
            </a>
        </div>
    }
}

async fn fetch_offer(certificate: &CertificateData) -> Result<String, Box<dyn std::error::Error>> {
    log::info!("Fetching offer {:?}", certificate);
    let post_url = format!("{}/api/v1/certificates/offer", ISSUER_URL);
    let request = Request::post(&post_url)
        .header("Content-Type", "application/json")
        .json(certificate)?;

    let response = request.send().await?;
    let text = response.json().await?;
    log::debug!("Response: {:?}", text);
    Ok(text)
}

fn render_offer(url: String) -> Html {
    // Generate the QR code
    let qr = QrCode::encode_text(&url, QrCodeEcc::Medium).unwrap();

    // Convert QR code to SVG string
    let svg = qr_code_to_svg_string(&qr);

    log::debug!("QR code: {:?}", svg);

    let parsed_html = Html::from_html_unchecked(AttrValue::from(svg));

    html! {
        <div class="qr-code">
            <a href={url} >
                { parsed_html }
            </a>
        </div>
    }
}

fn qr_code_to_svg_string(qr: &QrCode) -> String {
    let mut svg = String::new();
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 ");
    svg.push_str(&qr.size().to_string());
    svg.push_str(" ");
    svg.push_str(&qr.size().to_string());
    svg.push_str("\" stroke=\"none\">");
    svg.push_str("<path d=\"");
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    svg.push(' ');
                }
                svg.push_str(&format!("M{},{}h1v1h-1z", x, y));
            }
        }
    }
    svg.push_str("\" fill=\"#000000\"/>");
    svg.push_str("</svg>");
    svg
}

fn generate_performance_record() -> CertificateData {
    let web_session = WebSession::default();

    let profile = ProfileStorage::default().get("").unwrap_or_default();

    let challenge_history = web_session
        .session
        .game_state
        .game
        .challenge_history
        .clone();
    let profile_name = profile.name.clone();
    let game_paths = web_session.session.game_state.game.game_paths.clone();
    let current_level = web_session.session.game_state.current_game_path;

    let game_path_id = game_paths[current_level].id.clone();

    let total_challenges = game_paths[current_level].challenges.len();

    let performance_record = PerformanceRecord::new_from_history(
        game_path_id.clone(),
        profile_name.clone(),
        total_challenges,
        challenge_history.clone(),
    );

    let mut certificate_data = CertificateData::from(performance_record);
    certificate_data.create_signature();

    certificate_data
}
