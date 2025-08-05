use leptos::prelude::*;

use crate::i18n::*;


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
pub fn Stack() -> impl IntoView {
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
        Stack::new("Aiogram", "public/aiogram.svg", 18),
        Stack::new("Leptos", "https://simpleicons.org/icons/leptos.svg", 1),
        Stack::new("Axum", "https://simpleicons.org/icons/tokio.svg", 4),
        Stack::new("React", "https://simpleicons.org/icons/react.svg", 10),
        Stack::new("Next.js", "https://simpleicons.org/icons/nextdotjs.svg", 8),
        Stack::new("Gin", "https://simpleicons.org/icons/gin.svg", 1),
    ];
    let i18n = use_i18n();
    let locale = i18n.get_locale();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-x-4 pt-1 flex-1">
            <h1 class="text-xl dark:text-gray-100">{t!(i18n, stack.title)}</h1>
            <div class="flex flex-row justify-stretch">
                <div class="w-full flex flex-row gap-2 basis-2/3 p-4">
                    <ul>
                        {stack.iter().map(|s| view!{
                            <li class="flex flex-row gap-2 items-center">
                                <img src="public/point.svg" class="w-[10px] h-[10px]" />
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
                                        <div style=format!("width: {percentage}%; height: 4.3px") class=format!("{color}")></div>
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
