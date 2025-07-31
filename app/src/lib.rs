use leptos::{attr::Datetime, prelude::*};
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
                {description.split("\n").map(|s| view!{<p class="text-left">{s}</p>}).collect::<Vec<_>>()}
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
                <div class="w-full flex flex-row gap-2 basis-2/3 p-4">
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
                <div class="w-full basis-1/3 bg-gray-200 shadow rounded-2xl p-2 m-2 box-content">
                    <ul class="flex flex-col gap-5 p-2">
                        {
                            let max_experience = stack.iter().map(|s| s.experience).max().unwrap_or(0);
                            stack.iter().map(|s| {
                                let percentage = (s.experience as f32 / max_experience as f32) * 100.0;
                                let color = match percentage {
                                    0.0..=11.3 => "bg-red-700",
                                    11.4..=22.6 => "bg-red-500",
                                    22.7..=33.9 => "bg-red-400",
                                    34.0..=45.3 => "bg-yellow-600",
                                    45.4..=56.6 => "bg-yellow-500",
                                    56.7..=66.9 => "bg-yellow-300",
                                    67.0..=78.3 => "bg-green-700",
                                    78.4..=89.6 => "bg-green-500",
                                    _ => "bg-green-300", // 89.7..=100.0 and any percentage > 100.0
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

struct Step {
    title: &'static str,
    description: &'static str,
}

impl Step {
    fn new(title: &'static str, description: &'static str) -> Self {
        Self { title, description }
    }
}

#[component]
fn Process() -> impl IntoView {
    let steps = vec![
        Step::new(
            "Project Discussion",
            "We chat to clarify your goals, features, timeline and budget. You can share references, sketches, or anything helpful."
        ),
        Step::new(
            "Proposal & Agreement",
            "I send a clear project outline with scope, timeline, and price. Once confirmed, we move forward."
        ),
        Step::new(
            "Upfront Payment & Start",
            "A 30–50% upfront payment secures the project. I begin work based on the agreed plan."
        ),
        Step::new(
            "Progress Updates & Feedback",
            "You receive updates at key milestones. I welcome feedback and make any needed adjustments."
        ),
        Step::new(
            "Final Delivery & Payment",
            "After final approval, you make the remaining payment. I deliver everything as promised — ready to launch or hand over."
        )
    ];
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl">How does work process go</h1>
            <div class="flex flex-col justify-stretch">
                {steps.iter().enumerate().map(|(i, s)| view!{
                    <div class="flex flex-col">
                        <div class="flex flex-row gap-1">
                            <p>{i + 1}.</p>
                            <p class="font-bold">{s.title}</p>
                        </div>
                        <p class="text-right text-gray-600">{s.description}</p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

struct Experience {
    pub orderer: &'static str,
    pub description: &'static str,
    pub role: &'static str,
    pub start: &'static str,
    pub end: &'static str,
}

impl Experience {
    fn new(
        orderer: &'static str,
        description: &'static str,
        role: &'static str,
        start: &'static str,
        end: &'static str,
    ) -> Self {
        Self {
            orderer,
            description,
            role,
            start,
            end,
        }
    }
}

#[component]
fn Experiences() -> impl IntoView {
    let experiences = vec![
        Experience::new(
            "Dead Inside",
            r#"
                Delivered product successuffuly after payment,
                then fixes, new updates and still available on
                Telegram
            "#,
            "Bot developer",
            "01.01.2023",
            "01.01.2024",
        ),
        Experience::new(
            "IT Programmer",
            r#"
                Some random text, because I can't really remember,
                it was quite long ago and I don't want to remember it
                anymore
            "#,
            "Fullstack developer",
            "01.06.2024",
            "01.09.2024",
        ),
        Experience::new(
            "IT Programmer",
            r#"
                Some random text, because I can't really remember,
                it was quite long ago and I don't want to remember it
                anymore
            "#,
            "Fullstack developer",
            "01.06.2024",
            "01.09.2024",
        ),
    ];
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl">Experience</h1>
            <div class="flex flex-col justify-center align-items h-full w-full gap-4">
                {experiences.iter().map(|e| view!{
                    <div class="flex flex-row gap-1 rounded-3xl border-black border p-4">
                        <div class="flex flex-col gap-1">
                            <p>{e.orderer}</p>
                            <p class="text-gray-600 text-sm">{e.start} - {e.end}</p>
                        </div>
                        <div class="flex flex-col justify-around">
                            <p class="text-right">{e.description}</p>
                            <p class="text-gray-600">{e.role}</p>
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
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

struct Contact {
    platform: &'static str,
    link: &'static str,
    icon: &'static str,
    value: &'static str,
}

impl Contact {
    fn new(
        platform: &'static str,
        link: &'static str,
        icon: &'static str,
        value: &'static str,
    ) -> Self {
        Self {
            platform,
            link,
            icon,
            value,
        }
    }
}

#[component]
fn Contacts() -> impl IntoView {
    let contacts = vec![
        Contact::new(
            "GitHub",
            "https://github.com/dextermorgan",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/github.svg",
            "dextermorgan",
        ),
        Contact::new(
            "Email",
            "mailto:OoN7l@example.com",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/gmail.svg",
            "OoN7l@example.com",
        ),
        Contact::new(
            "Telegram",
            "https://t.me/dextermorgan",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/telegram.svg",
            "@dextermorgan",
        ),
        Contact::new(
            "Twitter (X)",
            "https://twitter.com/dextermorgan",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/x.svg",
            "@dextermorgan",
        ),
        Contact::new(
            "LinkedIn",
            "https://linkedin.com/in/dextermorgan",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/linkedin.svg",
            "dextermorgan",
        ),
        Contact::new(
            "Whatsapp",
            "https://wa.me/6281312345678",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/whatsapp.svg",
            "+6281312345678",
        ),
    ];
    let details = r#"
        Typically reply within 24h
        Open to freelance projects — Mon to Fri
        Timezone: GMT+3
    "#;
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl">Contacts</h1>
            <div class="flex flex-col justify-center align-items h-full w-full">
                {contacts.iter().map(|c| view!{
                    <a href={c.link}>
                        <div class="flex flex-row gap-1">
                            <img src={c.icon} class="w-[20px] h-[20px]" />
                            <p>{c.platform}</p>
                            <p>" — "{c.value}</p>
                        </div>
                    </a>
                }).collect::<Vec<_>>()}
                <div style="flex-basis: 5%; height: 0;"></div>
                {details.split("\n").map(|s| view!{<p class="text-left">{s}</p>}).collect::<Vec<_>>()}
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
                <Experiences />
                <Projects />
                <Contacts />
            </div>
        </div>
    }
}
