use leptos::prelude::*;

use crate::i18n::*;

#[derive(Clone)]
struct StackItem {
    name: &'static str,
    icon: &'static str,
    details: &'static str,
}

impl StackItem {
    pub fn new(name: &'static str, icon: &'static str, details: &'static str) -> Self {
        Self {
            name,
            icon,
            details,
        }
    }
}

#[component]
pub fn Stack() -> impl IntoView {
    let i18n = use_i18n();

    let languages = vec![
        StackItem::new(
            "Golang",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/go.svg",
            t_string!(i18n, stack.stack_go_desc),
        ),
        StackItem::new(
            "Rust",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/rust.svg",
            t_string!(i18n, stack.stack_rust_desc),
        ),
        StackItem::new(
            "Python",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/python.svg",
            t_string!(i18n, stack.stack_python_desc),
        ),
        StackItem::new(
            "TypeScript",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/typescript.svg",
            t_string!(i18n, stack.stack_ts_desc),
        ),
    ];

    let databases = vec![
        StackItem::new(
            "PostgreSQL",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/postgresql.svg",
            t_string!(i18n, stack.stack_pg_desc),
        ),
        StackItem::new(
            "CockroachDB",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/cockroachlabs.svg",
            t_string!(i18n, stack.stack_crdb_desc),
        ),
        StackItem::new(
            "ClickHouse",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/clickhouse.svg",
            t_string!(i18n, stack.stack_ch_desc),
        ),
        StackItem::new(
            "Redis",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/redis.svg",
            t_string!(i18n, stack.stack_redis_desc),
        ),
        StackItem::new(
            "ScyllaDB",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/scylladb.svg",
            t_string!(i18n, stack.stack_scylla_desc),
        ),
    ];

    let infra = vec![
        StackItem::new(
            "Docker",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/docker.svg",
            t_string!(i18n, stack.stack_docker_desc),
        ),
        StackItem::new(
            "Kubernetes",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/kubernetes.svg",
            t_string!(i18n, stack.stack_k8s_desc),
        ),
        StackItem::new(
            "Apache Kafka",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/apachekafka.svg",
            t_string!(i18n, stack.stack_kafka_desc),
        ),
        StackItem::new(
            "gRPC",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/cncf.svg",
            t_string!(i18n, stack.stack_grpc_desc),
        ),
        StackItem::new(
            "Linux",
            "https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/linux.svg",
            t_string!(i18n, stack.stack_linux_desc),
        ),
    ];

    view! {
        <div class="flex flex-col gap-8 p-6 bg-gray-100 dark:bg-gray-800 rounded-3xl shadow-xl">
            <StackSection title=t_string!(i18n, stack.programming_languages) items=languages />
            <StackSection title=t_string!(i18n, stack.databases) items=databases />
            <StackSection title=t_string!(i18n, stack.infrastructure) items=infra />
        </div>
    }
}

#[component]
fn StackSection(title: &'static str, items: Vec<StackItem>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <h2 class="text-sm font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400 ml-2">
                {title}
            </h2>
            <div class="flex flex-wrap gap-3">
                {items.into_iter().map(|item| view! { <StackCard item=item /> }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn StackCard(item: StackItem) -> impl IntoView {
    view! {
        <div class="group relative flex items-center justify-center p-3 w-14 h-14
                    bg-gray-200 dark:bg-gray-900 rounded-xl transition-all duration-300
                    hover:scale-110 hover:shadow-lg hover:bg-indigo-50 dark:hover:bg-indigo-900/30
                    cursor-pointer
                    ">

            // Иконка
            <img src=item.icon class="w-8 h-8 object-contain dark:invert grayscale group-hover:grayscale-0 transition-all" />

            // Поп-ап (Tooltip)
            <div class="absolute bottom-full mb-3 hidden group-hover:flex flex-col items-center z-50 animate-in fade-in zoom-in duration-200">
                <div class="bg-gray-900 dark:bg-gray-100 text-white dark:text-gray-900 text-xs py-2 px-3 rounded-lg shadow-xl whitespace-nowrap">
                    <span class="font-bold">{item.name}</span>
                    <span class="mx-1 opacity-50">|</span>
                    <span>{item.details}</span>
                </div>
                // Треугольничек внизу поп-апа
                <div class="w-2 h-2 bg-gray-900 dark:bg-gray-100 rotate-45 -mt-1"></div>
            </div>
        </div>
    }
}
