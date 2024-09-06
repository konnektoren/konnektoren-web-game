use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/map")]
    Map,
    #[at("/about")]
    About,
    #[at("/ads")]
    Ads,
    #[at("/challenge/:id")]
    Challenge { id: String },
    #[at("/challenges")]
    Challenges,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/profile")]
    Profile,
    #[at("/results/:code")]
    Results { code: String },
    #[at("/payment")]
    Payment,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[cfg(feature = "yew-preview")]
    #[at("/preview")]
    YewPreview,
}

impl From<&str> for Route {
    fn from(query: &str) -> Self {
        if query.contains("page=about") {
            return Route::About;
        }

        if query.contains("page=challenge") {
            let id = query
                .split('&')
                .find(|part| part.starts_with("id="))
                .and_then(|id_part| id_part.split('=').nth(1))
                .unwrap_or("");

            return Route::Challenge { id: id.to_string() };
        }
        if query.contains("page=challenges") {
            return Route::Challenges;
        }
        if query.contains("page=leaderboard") {
            return Route::Leaderboard;
        }

        if query.contains("page=ads") {
            return Route::Ads;
        }
        if query.contains("page=map") {
            return Route::Map;
        }
        if query.contains("page=profile") {
            return Route::Profile;
        }
        if query.contains("page=settings") {
            return Route::Settings;
        }

        if query.contains("page=results") {
            let code = query
                .split('&')
                .find(|part| part.starts_with("code="))
                .and_then(|code_part| code_part.split('=').nth(1))
                .unwrap_or("");

            return Route::Results {
                code: code.to_string(),
            };
        }

        Route::Home
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_query() {
        assert_eq!(Route::from("page=about"), Route::About);
        assert_eq!(Route::from("page=profile"), Route::Profile);
        assert_eq!(
            Route::from("page=results&code=123"),
            Route::Results {
                code: "123".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=456"),
            Route::Results {
                code: "456".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=789"),
            Route::Results {
                code: "789".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=abc"),
            Route::Results {
                code: "abc".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=def"),
            Route::Results {
                code: "def".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=ghi"),
            Route::Results {
                code: "ghi".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=jkl"),
            Route::Results {
                code: "jkl".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=mno"),
            Route::Results {
                code: "mno".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=pqr"),
            Route::Results {
                code: "pqr".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=stu"),
            Route::Results {
                code: "stu".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=vwx"),
            Route::Results {
                code: "vwx".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=yz"),
            Route::Results {
                code: "yz".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=1234567890"),
            Route::Results {
                code: "1234567890".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=abcdefghijklmnopqrstuvwxyz"),
            Route::Results {
                code: "abcdefghijklmnopqrstuvwxyz".to_string()
            }
        );
        assert_eq!(
            Route::from("page=results&code=ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            Route::Results {
                code: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()
            }
        );
        assert_eq!(
            Route::from("page=challenge&id=123"),
            Route::Challenge {
                id: "123".to_string()
            }
        );
    }
}
