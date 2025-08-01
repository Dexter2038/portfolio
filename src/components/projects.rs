use leptos::prelude::*;

use crate::i18n::*;


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
pub fn Projects() -> impl IntoView {
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
