use leptos::logging::*;
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
    let (title, _setTitle) = signal("Portfolio".to_string());

    view! {
        <Title text=move || title.get()/>

        <Router>
            <main>
                <Routes fallback=|| view!{ <p>"Page not found."</p> }>
                    <Route path=StaticSegment("") view=move || view!{ <MainPage /> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Personal() -> impl IntoView {
    let name = "Michael Barkaloff";
    let role = "A software developer";
    let tech = r#"
        Proficient in Rust, Python, JS, TS,
        Golang, SQL, PostgreSQL, MySQL,
        FastAPI, Axum, Clap, TailwindCSS,
        Leptos, Aiogram, Teloxide, Gin,
        React, Next.js, REST, gRPC
    "#;
    let description = r#"
        Since my child years I have been making lego models
        In school I was interested in computers and games
        In college I got into an IT subject
        But it was a cybersecurity and there was not much code
        So, I decided to learn Python in parallel
        After 2 years of experience with programming
        I know a lot, both backend and frontend, and much more
        And now I am here
    "#;
    view! {
        <div class="flex flex-col min-w-[300px] max-w-[550px] min-h-[475px] border-black border-2 rounded-3xl p-4">
            <div class="flex flex-row justify-between">
                <img src="/portfolio-photo.jpg" class="w-[200px] h-[200px] rounded-3xl" />
                <div class="flex flex-col justify-center w-full">
                    <p class="text-gray-600">{name}</p>
                    <p class="text-gray-600">{role}</p>
                    <div style="flex-basis: 5%; height: 0;"></div>
                    {tech.split("\n").map(|s| view!{<p class="text-gray-800">{s}</p>}).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="flex flex-col justify-center align-items h-full">
                {description.split("\n").map(|s| view!{<p>{s}</p>}).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn Stack() -> impl IntoView {
    view! {
        <h1>"Stack"</h1>
    }
}

#[component]
fn Process() -> impl IntoView {
    view! {
        <h1>"Process"</h1>
    }
}

#[component]
fn Experience() -> impl IntoView {
    view! {
        <h1>"Experience"</h1>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <h1>"Projects"</h1>
    }
}

#[component]
fn Contacts() -> impl IntoView {
    view! {
        <h1>"Contacts"</h1>
    }
}

#[component]
fn MainPage() -> impl IntoView {
    view! {
        // Tailwind
        <div class="flex flex-col flex-wrap h-screen w-screen m-4">
            <div class="flex flex-col flex-wrap">
                <Personal />
                <Stack />
                <Process />
            </div>
            <div class="flex flex-col">
                <Experience />
                <Projects />
                <Contacts />
            </div>
        </div>
    }
}
