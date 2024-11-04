pub mod about;
pub mod achievements;

#[cfg(feature = "backup")]
pub mod backup;
pub mod challenge;
pub mod home;
pub mod map;
pub mod profile;

mod challenges;
mod leaderboard;
mod marketplace;
#[cfg(feature = "yew-preview")]
pub mod preview;
mod results;
mod search;
mod settings;

pub use about::AboutPage;
pub use achievements::AchievementsPage;
#[cfg(feature = "backup")]
pub use backup::BackupPage;
pub use challenge::ChallengePage;
pub use challenges::ChallengesPage;
pub use home::HomePage;
pub use leaderboard::LeaderboardPage;
pub use map::MapPage;
pub use marketplace::MarketplacePage;
pub use profile::ProfilePage;
pub use results::ResultsPage;
pub use search::SearchPage;
pub use settings::SettingsPage;

pub use routing::prelude::NotFoundPage;
