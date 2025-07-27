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
            <main class="h-screen w-screen p-4 bg-gray-200">
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
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-4 flex-1">
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
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-x-4 flex-1">
            <h1 class="text-xl">Stack</h1>
            <div class="flex flex-row justify-stretch">
                <ul class="w-full basis-2/3">
                    <li>Leptos</li>
                    <li>Tailwind</li>
                    <li>Axum</li>
                    <li>FastAPI</li>
                    <li>React</li>
                    <li>Next.js</li>
                </ul>
                <div class="w-full basis-1/3">
                    <ul>
                        <li>Leptos</li>
                        <li>Tailwind</li>
                        <li>Axum</li>
                        <li>FastAPI</li>
                        <li>React</li>
                        <li>Next.js</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Process() -> impl IntoView {
    view! {
        <div class="flex flex-col basis-1/3 border-black  bg-gray-100 shadow rounded-3xl p-4 flex-1">
            <div class="flex flex-row justify-between">
                <img src="/portfolio-photo.jpg" class="w-[200px] h-[200px] rounded-3xl" />
                <div class="flex flex-col justify-center w-full">
                </div>
            </div>
            <div class="flex flex-col justify-center align-items h-full">
                Process
            </div>
        </div>
    }
}

#[component]
fn Experience() -> impl IntoView {
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-4 flex-1">
            <div class="flex flex-row justify-between">
                <img src="/portfolio-photo.jpg" class="w-[200px] h-[200px] rounded-3xl" />
                <div class="flex flex-col justify-center w-full">
                </div>
            </div>
            <div class="flex flex-col justify-center align-items h-full">
                Experience
            </div>
        </div>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-4 flex-1">
            <div class="flex flex-row justify-between">
                <img src="/portfolio-photo.jpg" class="w-[200px] h-[200px] rounded-3xl" />
                <div class="flex flex-col justify-center w-full">
                </div>
            </div>
            <div class="flex flex-col justify-center align-items h-full">
                Projects
            </div>
        </div>
    }
}

#[component]
fn Contacts() -> impl IntoView {
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-4 flex-1">
            <div class="flex flex-row justify-between">
                <img src="/portfolio-photo.jpg" class="w-[200px] h-[200px] rounded-3xl" />
                <div class="flex flex-col justify-center w-full">
                </div>
            </div>
            <div class="flex flex-col justify-center align-items h-full">
                Contacts
            </div>
        </div>
    }
}

#[component]
fn MainPage() -> impl IntoView {
    view! {
        // Tailwind
        <div class="flex flex-col flex-wrap gap-4 w-full h-full">
            <Personal />
                <Experience />
            <Stack />
                <Projects />
            <Process />
                <Contacts />
        </div>
    }
}
