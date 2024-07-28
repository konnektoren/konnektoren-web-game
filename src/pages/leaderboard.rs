use gloo::utils::document;
use itertools::Itertools;
use konnektoren_core::challenges::PerformanceRecord;
use konnektoren_yew::i18n::use_i18n;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

const API_URL: &str = "https://api.konnektoren.help/api/v1/leaderboard";

#[derive(Serialize, Deserialize)]
pub struct LeaderboardV1Response {
    pub performance_records: Vec<PerformanceRecord>,
}

pub async fn fetch_all_performance_records() -> Vec<PerformanceRecord> {
    let response = reqwest::get(API_URL).await.unwrap();
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

#[function_component(LeaderboardPage)]
pub fn leaderboard_page() -> Html {
    let i18n = use_i18n();
    let title = format!("Konnektoren - {}", i18n.t("Leaderboard"));
    use_effect(move || {
        document().set_title(&title);
        || ()
    });

    let leaderboard = use_state(|| Vec::<PerformanceRecord>::new());
    {
        let leaderboard = leaderboard.clone();
        use_effect_with((), |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let performance_records = fetch_all_performance_records().await;
                leaderboard.set(performance_records);
            });
        });
    }

    html! {
        <div class="leaderboard-page">
            <h1>{"Leaderboard"}</h1>
            <div class="leaderboard">
                <table>
                    <thead>
                        <tr>
                            <th>{"Rank"}</th>
                            <th>{"Name"}</th>
                            <th>{"Performance"}</th>
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
        </div>
    }
}
