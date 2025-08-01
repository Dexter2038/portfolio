use leptos::prelude::*;

use crate::i18n::*;


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
pub fn Experiences() -> impl IntoView {
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
