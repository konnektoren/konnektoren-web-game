use chrono::Duration;
use itertools::Itertools;
use konnektoren_core::challenges::{PerformanceRecord, Timed};
use konnektoren_yew::components::TimerComponent;
use konnektoren_yew::i18n::use_i18n;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardProps {
    #[prop_or_default]
    pub leaderboard_id: Option<String>,

    #[prop_or_default]
    pub default_record: Option<PerformanceRecord>,
}

const API_URL: &str = "https://api.konnektoren.help/api/v1/leaderboard";

#[derive(Serialize, Deserialize)]
pub struct LeaderboardV1Response {
    pub performance_records: Vec<PerformanceRecord>,
}

pub async fn fetch_all_performance_records(
    leaderboard_id: Option<String>,
) -> Vec<PerformanceRecord> {
    let url = match leaderboard_id {
        Some(id) => format!("{}/{}", API_URL, id),
        None => API_URL.to_string(),
    };
    let response = reqwest::get(url).await.unwrap();
    let leaderboard: LeaderboardV1Response = response.json().await.unwrap();

    let performance_records = leaderboard.performance_records;

    // Sort by performance then by elapsed time (least time first)
    let sorted_performance_records = performance_records
        .into_iter()
        .sorted_by(|a, b| {
            b.performance_percentage
                .partial_cmp(&a.performance_percentage)
                .unwrap()
                .then(
                    // Get elapsed time for each record
                    a.elapsed_time()
                        .unwrap_or(Duration::hours(1))
                        .num_milliseconds()
                        .cmp(
                            &b.elapsed_time()
                                .unwrap_or(Duration::hours(1))
                                .num_milliseconds(),
                        ),
                )
        })
        .collect();
    sorted_performance_records
}

#[function_component(LeaderboardComp)]
pub fn leaderboard_comp(props: &LeaderboardProps) -> Html {
    let i18n = use_i18n();
    let leaderboard = use_state(|| match props.default_record.clone() {
        Some(record) => vec![record],
        None => vec![],
    });
    {
        let default_record = props.default_record.clone();
        let leaderboard = leaderboard.clone();
        let leaderboard_id = props.leaderboard_id.clone();
        use_effect_with(leaderboard_id.clone(), |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let mut performance_records = fetch_all_performance_records(leaderboard_id).await;
                if let Some(default_record) = default_record {
                    performance_records.push(default_record);
                }
                performance_records.sort();
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
                        <th>{i18n.t("Time")}</th>
                    </tr>
                </thead>
                <tbody>
                    { for leaderboard.iter().enumerate().map(|(i, record)| {
                        let elapsed_time = record.elapsed_time().unwrap_or_default().num_milliseconds();
                        log::info!("Elapsed time: {}", elapsed_time);
                        html! {
                            <tr>
                                <td>{i + 1}</td>
                                <td>{&record.profile_name}</td>
                                <td>{format!("{:.2}%", record.performance_percentage)}</td>
                                <td><TimerComponent milliseconds={elapsed_time} show_milliseconds={true} /></td>
                            </tr>
                        }
                    }) }
                </tbody>
            </table>
        </div>
    }
}
