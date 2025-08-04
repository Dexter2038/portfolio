use leptos::prelude::*;

use crate::i18n::*;


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
pub fn Contacts() -> impl IntoView {
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
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">{t_string!(i18n, contacts.title)}</h1>
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
                {t_string!(i18n, contacts.details).split("\n").map(|s| view!{<p class="text-left dark:text-gray-100">{s}</p>}).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
