use leptos::prelude::*;

use crate::i18n::*;

#[component]
pub fn Process() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">{t!(i18n, process.title)}</h1>
            <div class="flex flex-col justify-stretch">
                <div class="flex flex-col">
                    <div class="flex flex-row gap-1">
                        <p class="font-bold dark:text-gray-100">1.</p>
                        <p class="font-bold dark:text-gray-100">{t!(i18n, process.step_0_title)}</p>
                    </div>
                    <p class="text-right text-gray-600 dark:text-gray-200">{t!(i18n, process.step_0_description)}</p>
                </div>
                <div class="flex flex-col">
                    <div class="flex flex-row gap-1">
                        <p class="font-bold dark:text-gray-100">2.</p>
                        <p class="font-bold dark:text-gray-100">{t!(i18n, process.step_1_title)}</p>
                    </div>
                    <p class="text-right text-gray-600 dark:text-gray-200">{t!(i18n, process.step_1_description)}</p>
                </div>
                <div class="flex flex-col">
                    <div class="flex flex-row gap-1">
                        <p class="font-bold dark:text-gray-100">3.</p>
                        <p class="font-bold dark:text-gray-100">{t!(i18n, process.step_2_title)}</p>
                    </div>
                    <p class="text-right text-gray-600 dark:text-gray-200">{t!(i18n, process.step_2_description)}</p>
                </div>
                <div class="flex flex-col">
                    <div class="flex flex-row gap-1">
                        <p class="font-bold dark:text-gray-100">4.</p>
                        <p class="font-bold dark:text-gray-100">{t!(i18n, process.step_3_title)}</p>
                    </div>
                    <p class="text-right text-gray-600 dark:text-gray-200">{t!(i18n, process.step_3_description)}</p>
                </div>
                <div class="flex flex-col">
                    <div class="flex flex-row gap-1">
                        <p class="font-bold dark:text-gray-100">5.</p>
                        <p class="font-bold dark:text-gray-100">{t!(i18n, process.step_4_title)}</p>
                    </div>
                    <p class="text-right text-gray-600 dark:text-gray-200">{t!(i18n, process.step_4_description)}</p>
                </div>
            </div>
        </div>
    }
}
