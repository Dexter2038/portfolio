#![allow(deprecated)]
leptos_i18n::load_locales!();
use i18n::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_use::use_interval_fn;
//use leptos_router::{components::*, path};

// Modules
mod components;
mod home;

// Top-Level pages
use crate::home::Home;
//use crate::pages::not_found::NotFound;


#[derive(Clone)]
enum TitleState {
    Add,
    Remove,
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (title, set_title) = signal("Welcome".to_string());
    let (title_count, set_title_count) = signal(0);
    let (direction, set_direction) = signal(TitleState::Add);
    let max_title_count = 5;

    use_interval_fn(move || {
        set_title.update(|title| {
            *title = format!("Welcome{}", "!".repeat(title_count.get()));
        });
        match direction.get() {
            TitleState::Add => {
                set_title_count.update(|title_count| *title_count += 1);
                if title_count.get() >= max_title_count {
                    set_direction.update(|dir| *dir = TitleState::Remove);
                }
            }
            TitleState::Remove => {
                set_title_count.update(|title_count| *title_count -= 1);
                if title_count.get() <= 0 {
                    set_direction.update(|dir| *dir = TitleState::Add);
                }
            }
        }
    }, 200);

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
