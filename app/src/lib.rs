use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| view!{ <p>"Page not found."</p> }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Personal() -> impl IntoView {
    view! {
        <h1>"Personal"</h1>
    }
}

#[component]
fn StackExperience() -> impl IntoView {
    view! {
        <h1>"Stack Experience"</h1>
    }
}

#[component]
fn ProcessDescription() -> impl IntoView {
    view! {
        <h1>"Process Description"</h1>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Personal />
        <StackExperience />
    }
}
