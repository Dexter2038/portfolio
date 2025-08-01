use leptos::prelude::*;
use web_sys::window;

use crate::i18n::{use_i18n, Locale};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
     let i18n = use_i18n();
    if let Some(window) = window() {
        let navigator = window.navigator();
        let langs = navigator.languages();
        if let Some(lang) = langs.get(0).as_string() {
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
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>                
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
        </ErrorBoundary>
    }
}
