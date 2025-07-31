#![allow(deprecated)]
leptos_i18n::load_locales!();
use i18n::*;
use leptos::{logging::log, prelude::*};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (title, _setTitle) = signal("Portfolio".to_string());

    view! {
        <Title text=move || title.get()/>


        <I18nContextProvider >
            <Router>
                <main class="h-screen w-screen p-4 dark:bg-gray-700 bg-gray-200">
                    <Routes fallback=|| view!{ <p>"Page not found."</p> }>
                        <Route path=StaticSegment("") view=move || view!{ <MainPage /> }/>
                    </Routes>
                </main>
            </Router>
        </I18nContextProvider>
    }
}

#[component]
fn Personal() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <div class="flex flex-row justify-between">
                <img src="/portfolio-photo.jpg" class="w-[200px] h-[200px] rounded-3xl" />
                <div class="flex flex-col justify-center w-full">
                    <p class="text-gray-600 dark:text-gray-200">{t!(i18n, personal.name)}</p>
                    <p class="text-gray-600 dark:text-gray-200">{t!(i18n, personal.role)}</p>
                    <div style="flex-basis: 5%; height: 0;"></div>
                    <p class="text-gray-800 dark:text-gray-400">{t!(i18n, personal.tech)}</p>
                </div>
            </div>
            <div class="flex flex-col justify-center align-items h-full">
                <p class="dark:text-gray-100 text-left">{t!(i18n, personal.description)}</p>
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
    let i18n = use_i18n();
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
    let i18n = use_i18n();
    let locale = i18n.get_locale();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-x-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">Stack</h1>
            <div class="flex flex-row justify-stretch">
                <div class="w-full flex flex-row gap-2 basis-2/3 p-4">
                    <ul>
                        {stack.iter().map(|s| view!{
                            <li class="flex flex-row gap-2 items-center">
                                <img src="/point.svg" class="w-[10px] h-[10px]" />
                                <p class="dark:text-white">{s.name}</p>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                    <ul class="flex flex-col">
                        {stack.iter().map(|s| view!{
                            <li class="flex flex-row gap-2 items-center">
                                <img src={s.icon} class="w-[20px] h-[20px] dark:invert" />
                                <p class="dark:text-white">{s.experience}" "{
                                    match locale {
                                    Locale::en => {
                                        match s.experience {
                                            1 => "month".into_view(),
                                            _ => "months".into_view(),
                                        }
                                    },
                                    Locale::ru => {
                                        let rem_10 = s.experience % 10;
                                        let rem_100 = s.experience % 100;

                                        let word = if rem_100 >= 11 && rem_100 <= 14 {
                                            "месяцев"
                                        } else {
                                            match rem_10 {
                                                1 => "месяц",
                                                2..=4 => "месяца",
                                                _ => "месяцев",
                                            }
                                        };

                                        word.into_view()
                                    }
                                }}
                                </p>
                            </li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
                <div class="w-full basis-1/3 bg-gray-200 dark:bg-gray-700 shadow rounded-2xl p-2 m-2 box-content">
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
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">How does work process go</h1>
            <div class="flex flex-col justify-stretch">
                {steps.iter().enumerate().map(|(i, s)| view!{
                    <div class="flex flex-col">
                        <div class="flex flex-row gap-1">
                            <p class="font-bold dark:text-gray-100">{i + 1}.</p>
                            <p class="font-bold dark:text-gray-100">{s.title}</p>
                        </div>
                        <p class="text-right text-gray-600 dark:text-gray-200">{s.description}</p>
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
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">Experience</h1>
            <div class="flex flex-col justify-center align-items h-full w-full gap-4">
                {experiences.iter().map(|e| view!{
                    <div class="flex flex-row gap-1 rounded-3xl bg-gray-200 dark:bg-gray-700 p-4">
                        <div class="flex flex-col gap-1">
                            <p class="font-bold dark:text-white">{e.orderer}</p>
                            <p class="text-gray-600 text-sm dark:text-gray-300">{e.start} - {e.end}</p>
                        </div>
                        <div class="flex flex-col justify-around">
                            <p class="text-right dark:text-gray-100">{e.description}</p>
                            <p class="text-gray-600 dark:text-gray-300">{e.role}</p>
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

struct Project {
    title: &'static str,
    description: &'static str,
    pic: &'static str,
    link: &'static str,
    tech: Vec<&'static str>,
}

impl Project {
    fn new(
        title: &'static str,
        description: &'static str,
        pic: &'static str,
        link: &'static str,
        tech: Vec<&'static str>,
    ) -> Self {
        Self {
            title,
            description,
            pic,
            link,
            tech,
        }
    }
}

#[component]
fn Projects() -> impl IntoView {
    let projects = vec![
        Project::new(
            "Dead Inside",
            "Some random text, because I can't really remember",
            "/portfolio-photo.jpg",
            "https://github.com/dextermorgan/DeadInside",
            vec!["Rust", "Telegram bot", "Web"],
        ),
        Project::new(
            "Dead Inside",
            "Some random text, because I can't really remember",
            "/portfolio-photo.jpg",
            "https://github.com/dextermorgan/DeadInside",
            vec!["Rust", "Telegram bot", "Web"],
        ),
    ];
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">Projects</h1>
            <div class="flex flex-row justify-center align-items h-full w-full gap-4">
                {projects.iter().map(|p| view!{
                    <div class="flex flex-col gap-1 rounded-3xl bg-gray-200 dark:bg-gray-700 p-4">
                        <div class="flex flex-col gap-1">
                            <p class="font-bold dark:text-white">{p.title}</p>
                            <p class="text-gray-600 text-sm dark:text-gray-300">{p.description}</p>
                        </div>
                        <img src={p.pic} class="w-[200px] h-[200px] rounded-3xl" />
                        <div class="flex flex-col justify-around">
                            <a href={p.link} class="underline dark:text-gray-100">Open</a>
                            <div class="flex flex-col gap-1">
                                {p.tech.iter().map(|t| view!{
                                    <div class="flex flex-row gap-1 items-center">
                                        <img src="/point.svg" class="w-[10px] h-[10px]" />
                                        <p class="text-left dark:text-gray-100">{*t}</p>
                                    </div>
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
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
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">Contacts</h1>
            <div class="flex flex-col justify-center align-items h-full w-full">
                {contacts.iter().map(|c| view!{
                    <a href={c.link}>
                        <div class="flex flex-row gap-1">
                            <img src={c.icon} class="w-[20px] h-[20px] invert" />
                            <p class="dark:text-gray-100">{c.platform}</p>
                            <p class="dark:text-gray-100">" — "{c.value}</p>
                        </div>
                    </a>
                }).collect::<Vec<_>>()}
                <div style="flex-basis: 5%; height: 0;"></div>
                {details.split("\n").map(|s| view!{<p class="text-left dark:text-gray-100">{s}</p>}).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn MainPage() -> impl IntoView {
    let i18n = use_i18n();
    if let Some(window) = window() {
        let navigator = window.navigator();
        let langs = navigator.languages();
        if let Some(lang) = langs.get(0).as_string() {
            log!("{lang}");
            let is_cyrillic = lang.contains("ru")  // Russian
                || lang.contains("uk")                   // Ukrainian
                || lang.contains("be")                   // Belarusian
                || lang.contains("kk")                   // Kazakh
                || lang.contains("tg")                   // Tajik
                || lang.contains("uz")                   // Uzbek
                || lang.contains("ky")                   // Kyrgyz
                || lang.contains("tk")                   // Turkmen
                || lang.contains("mn")                   // Mongolian
                || lang.contains("sr"); // Serbian
            let locale = if is_cyrillic { Locale::ru } else { Locale::en };
            i18n.set_locale(locale);
        }
    }

    view! {
        // Tailwind
        <div class="flex flex-col gap-4 w-full h-full">
            <div class="flex flex-col md:flex-row gap-4 w-full md:h-full">
                <Personal />
                <Stack />
                <Process />
            </div>
            <div class="flex flex-col md:flex-row gap-4 w-full md:h-full">
                <Experiences />
                <Projects />
                <Contacts />
            </div>
        </div>
    }
}
