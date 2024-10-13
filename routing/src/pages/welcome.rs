use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(WelcomePage)]
pub fn welcome_page() -> Html {
    html! {
        <div class="welcome-page">
            <div class="speech-bubble">
            <h1>{ "Welcome to Konnektoren!" }</h1>

            <div id="orange-container">
                <embed
                    src="/assets/images/Orange_Animated.svg"
                    type="image/svg+xml"
                />
            </div>

            <Link<Route> to={Route::Home}>
                <p class="tagline">{ "Your interactive path to mastering German grammar" }</p>
                {"Let's start"}
            </Link<Route>>
            </div>

            <section class="intro">
                <h2>{ "What is Konnektoren?" }</h2>
                <p>{ "Konnektoren is an engaging platform designed to help you learn and practice German grammar, from A1 to C1 levels. Our focus is on making grammar learning interactive, fun, and effective." }</p>
            </section>

            <section class="features">
                <h2>{ "What You'll Find Here" }</h2>
                <ul>
                    <li>{ "Interactive exercises on German connectives (Konnektoren)" }</li>
                    <li>{ "Practice with articles, pronouns, and other key grammar topics" }</li>
                    <li>{ "Gamified learning experience to keep you motivated" }</li>
                    <li>{ "Progress tracking and immediate feedback" }</li>
                    <li>{ "Comprehensive coverage from beginner to advanced levels" }</li>
                </ul>
            </section>

            <section class="getting-started">
                <h2>{ "Getting Started" }</h2>
                <p>{ "Ready to begin your German grammar adventure? Click the button below to start exploring our challenges and improve your skills!" }</p>
                <Link<Route> to={Route::Home} classes="start-button">
                    { "Let's Start Learning!" }
                </Link<Route>>
            </section>

            <section class="about">
                <h2>{ "About Us" }</h2>
                <p>{ "Konnektoren was created by language enthusiasts who understand the challenges of learning German grammar. Our goal is to make your learning journey smoother and more enjoyable." }</p>
            </section>

            <footer>
                <p>{ "© 2024 Konnektoren. All rights reserved." }</p>
                <nav>
                    <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                </nav>
            </footer>

        </div>
    }
}
