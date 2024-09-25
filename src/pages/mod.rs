pub mod about;
pub mod challenge;
pub mod home;
pub mod map;
pub mod profile;

mod challenges;
mod leaderboard;
mod marketplace;
mod not_found;
#[cfg(feature = "yew-preview")]
pub mod preview;
mod results;
mod search;
mod settings;

pub use about::AboutPage;
pub use challenge::ChallengePage;
pub use challenges::ChallengesPage;
pub use home::HomePage;
pub use leaderboard::LeaderboardPage;
pub use map::MapPage;
pub use marketplace::MarketplacePage;
pub use not_found::NotFoundPage;
pub use profile::ProfilePage;
pub use results::ResultsPage;
pub use search::SearchPage;
pub use settings::SettingsPage;
