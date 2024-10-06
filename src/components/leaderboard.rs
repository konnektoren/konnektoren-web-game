use itertools::Itertools;
use konnektoren_core::challenges::PerformanceRecord;
use konnektoren_yew::i18n::use_i18n;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardProps {
    #[prop_or_default]
    pub challenge: Option<String>,
}

const API_URL: &str = "https://api.konnektoren.help/api/v1/leaderboard";

#[derive(Serialize, Deserialize)]
pub struct LeaderboardV1Response {
    pub performance_records: Vec<PerformanceRecord>,
}

pub async fn fetch_all_performance_records(challenge: Option<String>) -> Vec<PerformanceRecord> {
    let url = match challenge {
        Some(challenge) => format!("{}/{}", API_URL, challenge),
        None => API_URL.to_string(),
    };
    let response = reqwest::get(url).await.unwrap();
    let leaderboard: LeaderboardV1Response = response.json().await.unwrap();

    let performance_records = leaderboard.performance_records;

    let sorted_performance_records = performance_records
        .into_iter()
        .sorted_by(|a, b| {
            b.performance_percentage
                .partial_cmp(&a.performance_percentage)
                .unwrap()
        })
        .collect();
    sorted_performance_records
}

#[function_component(LeaderboardComp)]
pub fn leaderboard_comp(props: &LeaderboardProps) -> Html {
    let i18n = use_i18n();
    let leaderboard = use_state(|| Vec::<PerformanceRecord>::new());
    {
        let leaderboard = leaderboard.clone();
        let challenge = props.challenge.clone();
        use_effect_with(challenge.clone(), |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let performance_records = fetch_all_performance_records(challenge).await;
                leaderboard.set(performance_records);
            });
        });
    }

    if leaderboard.is_empty() {
        return html! {
            <div class="leaderboard">
            </div>
        };
    }

    html! {
        <div class="leaderboard">
            <table>
                <thead>
                    <tr>
                        <th>{i18n.t("Rank")}</th>
                        <th>{i18n.t("Name")}</th>
                        <th>{i18n.t("Performance")}</th>
                    </tr>
                </thead>
                <tbody>
                    { for leaderboard.iter().enumerate().map(|(i, record)| {
                        html! {
                            <tr>
                                <td>{i + 1}</td>
                                <td>{&record.profile_name}</td>
                                <td>{format!("{:.2}%", record.performance_percentage)}</td>
                            </tr>
                        }
                    }) }
                </tbody>
            </table>
        </div>
    }
}
