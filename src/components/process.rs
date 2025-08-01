use leptos::prelude::*;

use crate::i18n::*;


struct Step {
    title: &'static str,
    description: &'static str,
}

impl Step {
    fn new(title: &'static str, description: &'static str) -> Self {
        Self { title, description }
    }
}

#[component]
pub fn Process() -> impl IntoView {
    let steps = vec![
        Step::new(
            "Project Discussion",
            "We chat to clarify your goals, features, timeline and budget. You can share references, sketches, or anything helpful."
        ),
        Step::new(
            "Proposal & Agreement",
            "I send a clear project outline with scope, timeline, and price. Once confirmed, we move forward."
        ),
        Step::new(
            "Upfront Payment & Start",
            "A 30–50% upfront payment secures the project. I begin work based on the agreed plan."
        ),
        Step::new(
            "Progress Updates & Feedback",
            "You receive updates at key milestones. I welcome feedback and make any needed adjustments."
        ),
        Step::new(
            "Final Delivery & Payment",
            "After final approval, you make the remaining payment. I deliver everything as promised — ready to launch or hand over."
        )
    ];
    view! {
        <div class="flex flex-col basis-1/3 border-black bg-gray-100 dark:bg-gray-800 shadow rounded-3xl p-4 flex-1">
            <h1 class="text-xl dark:text-gray-100">How does work process go</h1>
            <div class="flex flex-col justify-stretch">
                {steps.iter().enumerate().map(|(i, s)| view!{
                    <div class="flex flex-col">
                        <div class="flex flex-row gap-1">
                            <p class="font-bold dark:text-gray-100">{i + 1}.</p>
                            <p class="font-bold dark:text-gray-100">{s.title}</p>
                        </div>
                        <p class="text-right text-gray-600 dark:text-gray-200">{s.description}</p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
