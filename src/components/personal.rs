use leptos::prelude::*;

use crate::i18n::*;


#[component]
pub fn Personal() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <div class="flex flex-row justify-between">
                <img src="public/portfolio-pic.png" class="w-[200px] h-[200px] rounded-3xl" />
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
