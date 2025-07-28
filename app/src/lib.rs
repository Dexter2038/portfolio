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

struct Stack {
    pub name: &'static str,
    pub icon: &'static str,
    pub experience: i32,
}

impl Stack {
    fn new(name: &'static str, icon: &'static str, experience: i32) -> Self {
        Self {
            name,
            icon,
            experience,
        }
    }
}

#[component]
fn Stack() -> impl IntoView {
    let stack = vec![
        Stack::new("Python", "https://simpleicons.org/icons/python.svg", 24),
        Stack::new(
            "JavaScript",
            "https://simpleicons.org/icons/javascript.svg",
            12,
        ),
        Stack::new(
            "TypeScript",
            "https://simpleicons.org/icons/typescript.svg",
            10,
        ),
        Stack::new("Rust", "https://simpleicons.org/icons/rust.svg", 5),
        Stack::new("Golang", "https://simpleicons.org/icons/go.svg", 2),
        Stack::new(
            "PostgreSQL",
            "https://simpleicons.org/icons/postgresql.svg",
            18,
        ),
        Stack::new("Docker", "https://simpleicons.org/icons/docker.svg", 10),
        Stack::new("Linux", "https://simpleicons.org/icons/linux.svg", 7),
        Stack::new("Git", "https://simpleicons.org/icons/git.svg", 24),
        Stack::new("Redis", "https://simpleicons.org/icons/redis.svg", 15),
        Stack::new("FastAPI", "https://simpleicons.org/icons/fastapi.svg", 20),
        Stack::new("Aiogram", "/aiogram.svg", 18),
        Stack::new("Leptos", "https://simpleicons.org/icons/leptos.svg", 1),
        Stack::new("Axum", "https://simpleicons.org/icons/tokio.svg", 4),
        Stack::new("React", "https://simpleicons.org/icons/react.svg", 10),
        Stack::new("Next.js", "https://simpleicons.org/icons/nextdotjs.svg", 8),
        Stack::new("Gin", "https://simpleicons.org/icons/gin.svg", 1),
    ];
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-x-4 flex-1">
            <h1 class="text-xl">Stack</h1>
            <div class="flex flex-row justify-stretch">
                <div class="w-full flex flex-row gap-2 basis-3/5 p-4">
                    <ul>
                        {stack.iter().map(|s| view!{
                            <li class="flex flex-row gap-2 items-center">
                                <img src="/point.svg" class="w-[20px] h-[20px]" />
                                <p>{s.name}</p>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                    <ul class="flex flex-col">
                        {stack.iter().map(|s| view!{
                            <li class="flex flex-row gap-2 items-center">
                                <img src={s.icon} class="w-[20px] h-[20px]" />
                                <p>{s.experience} year{if s.experience == 1 {""} else {"s"}}</p>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
                <div class="w-full basis-2/5 bg-gray-200 shadow rounded-2xl p-2 m-2 box-content">
                    <ul class="flex flex-col gap-5 p-2">
                        {
                            let max_experience = stack.iter().map(|s| s.experience).max().unwrap_or(0);
                            stack.iter().map(|s| {
                                let percentage = (s.experience as f32 / max_experience as f32) * 100.0;
                                let color = match percentage {
                                    0.0..=33.9 => "bg-red-500",
                                    34.0..=66.0 => "bg-yellow-500",
                                    _ => "bg-green-500",
                                };
                                view! {
                                    <li>
                                        <div class=format!("h-[4.3px] w-[{percentage}%] {color}")></div>
                                    </li>
                                }
                            }).collect::<Vec<_>>()
                        }
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
        <div class="flex flex-col gap-4 w-full h-full">
            <div class="flex flex-row gap-4 w-full h-full">
                <Personal />
                <Stack />
                <Process />
            </div>
            <div class="flex flex-row gap-4 w-full h-full">
                <Experience />
                <Projects />
                <Contacts />
            </div>
        </div>
    }
}
