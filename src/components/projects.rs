use leptos::prelude::*;

use crate::i18n::*;

#[component]
pub fn Projects() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">{t!(i18n, projects.title)}</h1>
            <div class="flex flex-row justify-center align-items h-full w-full gap-4">
                <div class="flex flex-col gap-1 rounded-3xl bg-gray-200 dark:bg-gray-700 p-4">
                    <div class="flex flex-col gap-1">
                        <p class="font-bold dark:text-white">{t!(i18n, projects.projects_0_title)}</p>
                        <p class="text-gray-600 text-sm dark:text-gray-300">{t!(i18n, projects.projects_0_description)}</p>
                    </div>
                    <a href={t_string!(i18n, projects.projects_0_link)}>
                        <img src=t_string!(i18n, projects.projects_0_pic) class="w-[200px] h-[200px] hover:ring-4 blur-lg hover:blur-none transition duration-300 ease-in-out rounded-3xl" />
                    </a>
                    <div class="flex flex-col justify-around">
                        <div class="flex flex-col gap-1">
                            <div class="flex flex-row gap-1 items-center">
                                <img src="public/point.svg" class="w-[10px] h-[10px]" />
                                <p class="text-left dark:text-gray-100">{t!(i18n, projects.projects_0_tech_1)}</p>
                            </div>
                            <div class="flex flex-row gap-1 items-center">
                                <img src="public/point.svg" class="w-[10px] h-[10px]" />
                                <p class="text-left dark:text-gray-100">{t!(i18n, projects.projects_0_tech_2)}</p>
                            </div>
                            <div class="flex flex-row gap-1 items-center">
                                <img src="public/point.svg" class="w-[10px] h-[10px]" />
                                <p class="text-left dark:text-gray-100">{t!(i18n, projects.projects_0_tech_3)}</p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col gap-1 rounded-3xl bg-gray-200 dark:bg-gray-700 p-4">
                    <div class="flex flex-col gap-1">
                        <p class="font-bold dark:text-white">{t!(i18n, projects.projects_1_title)}</p>
                        <p class="text-gray-600 text-sm dark:text-gray-300">{t!(i18n, projects.projects_1_description)}</p>
                    </div>
                    <a href={t_string!(i18n, projects.projects_1_link)}>
                        <img src=t_string!(i18n, projects.projects_1_pic) class="w-[200px] h-[200px] hover:ring-4 blur-lg hover:blur-none transition duration-300 ease-in-out rounded-3xl" />
                    </a>
                    <div class="flex flex-col justify-around">
                        <div class="flex flex-col gap-1">
                            <div class="flex flex-row gap-1 items-center">
                                <img src="public/point.svg" class="w-[10px] h-[10px]" />
                                <p class="text-left dark:text-gray-100">{t!(i18n, projects.projects_1_tech_1)}</p>
                            </div>
                            <div class="flex flex-row gap-1 items-center">
                                <img src="public/point.svg" class="w-[10px] h-[10px]" />
                                <p class="text-left dark:text-gray-100">{t!(i18n, projects.projects_1_tech_2)}</p>
                            </div>
                            <div class="flex flex-row gap-1 items-center">
                                <img src="public/point.svg" class="w-[10px] h-[10px]" />
                                <p class="text-left dark:text-gray-100">{t!(i18n, projects.projects_1_tech_3)}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
