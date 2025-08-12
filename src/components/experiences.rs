use leptos::prelude::*;

use crate::i18n::*;

#[component]
pub fn Experiences() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">{t!(i18n, experiences.title)}</h1>
            <div class="flex flex-col justify-center align-items h-full w-full gap-4">
                <div class="flex flex-row gap-1 rounded-3xl bg-gray-200 dark:bg-gray-700 p-4">
                    <div class="flex flex-col gap-1">
                        <p class="font-bold dark:text-white">"\""{t!(i18n, experiences.experiences_0_orderer)}"\""</p>
                        <p class="text-gray-600 text-sm dark:text-gray-300">{t!(i18n, experiences.experiences_0_start)} - {t!(i18n, experiences.experiences_0_end)}</p>
                    </div>
                    <div class="flex flex-col justify-around">
                        <p class="text-right dark:text-gray-100">{t!(i18n, experiences.experiences_0_description)}</p>
                        <p class="text-gray-600 dark:text-gray-300">{t!(i18n, experiences.experiences_0_role)}</p>
                    </div>
                </div>
                <div class="flex flex-row gap-1 rounded-3xl bg-gray-200 dark:bg-gray-700 p-4">
                    <div class="flex flex-col gap-1">
                        <p class="font-bold dark:text-white">"\""{t!(i18n, experiences.experiences_1_orderer)}"\""</p>
                        <p class="text-gray-600 text-sm dark:text-gray-300">{t!(i18n, experiences.experiences_1_start)} - {t!(i18n, experiences.experiences_1_end)}</p>
                    </div>
                    <div class="flex flex-col justify-around">
                        <p class="text-right dark:text-gray-100">{t!(i18n, experiences.experiences_1_description)}</p>
                        <p class="text-gray-600 dark:text-gray-300">{t!(i18n, experiences.experiences_1_role)}</p>
                    </div>
                </div>
                <div class="flex flex-row gap-1 rounded-3xl bg-gray-200 dark:bg-gray-700 p-4">
                    <div class="flex flex-col gap-1">
                        <p class="font-bold dark:text-white">"\""{t!(i18n, experiences.experiences_2_orderer)}"\""</p>
                        <p class="text-gray-600 text-sm dark:text-gray-300">{t!(i18n, experiences.experiences_2_start)} - {t!(i18n, experiences.experiences_2_end)}</p>
                    </div>
                    <div class="flex flex-col justify-around">
                        <p class="text-right dark:text-gray-100">{t!(i18n, experiences.experiences_2_description)}</p>
                        <p class="text-gray-600 dark:text-gray-300">{t!(i18n, experiences.experiences_2_role)}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
