#![allow(deprecated)]
leptos_i18n::load_locales!();
use i18n::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_use::use_interval_fn;
use rand::prelude::IndexedRandom;
//use leptos_router::{components::*, path};

// Modules
mod components;
mod home;

// Top-Level pages
use crate::home::Home;
//use crate::pages::not_found::NotFound;

#[derive(Clone, Copy, PartialEq)]
enum HackerState {
    Decoding,
    Wait,
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let phrases = vec![
        "INF: initialising raft cluster...",
        "INF: gRPC listening on :50051",
        "INF: kafka consumer group joined",
        "INF: cdc stream sync [ok]",
        "INF: clickhouse migration applied",
        "SYS: all nodes healthy (3/3)",
    ];

    let (title, set_title) = signal("LOADING".to_string());
    let (phrase_idx, set_phrase_idx) = signal(0);
    let (char_idx, set_char_idx) = signal(0);
    let (state, set_state) = signal(HackerState::Decoding);

    let charset = "ABC0123456789$#@%&*";

    use_interval_fn(
        move || {
            let current_phrase = phrases[phrase_idx.get()];

            match state.get() {
                HackerState::Decoding => {
                    let mut rng = rand::rng();
                    set_title.update(|t| {
                        let mut result = String::new();
                        for (i, target_char) in current_phrase.chars().enumerate() {
                            if i < char_idx.get() {
                                result.push(target_char); // Декодированная часть
                            } else if i == char_idx.get() {
                                // Случайный символ на месте текущего индекса
                                let random_char = charset.as_bytes().choose(&mut rng).unwrap();
                                result.push(*random_char as char);
                            } else {
                                result.push('·'); // Еще не тронутая часть
                            }
                        }
                        *t = result;
                    });

                    if char_idx.get() < current_phrase.len() {
                        set_char_idx.update(|i| *i += 1);
                    } else {
                        set_state.set(HackerState::Wait);
                        set_char_idx.set(0);
                    }
                }
                HackerState::Wait => {
                    // Пауза, чтобы успеть прочитать "SYSTEM_OK" и т.д.
                    if char_idx.get() < 10 {
                        set_char_idx.update(|i| *i += 1);
                    } else {
                        set_phrase_idx.update(|i| *i = (*i + 1) % phrases.len());
                        set_char_idx.set(0);
                        set_state.set(HackerState::Decoding);
                    }
                }
            }
        },
        120, // Скорость перебора символов
    );

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text=move || title.get()/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Body {..} class="h-full w-screen p-4 dark:bg-gray-700 bg-gray-200" id="body" />


        <I18nContextProvider>
            <Home />
        </I18nContextProvider>
    }
}
