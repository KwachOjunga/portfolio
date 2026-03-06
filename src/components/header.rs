use leptos::prelude::*;
// use leptos_theme::ThemeProvider;
use crate::components::themetoggle::ThemeToggle;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="fixed top-0 left-0 right-0 z-50 glass-effect backdrop-blur-md">
            <div class="max-w-4xl mx-auto px-4 py-4 flex justify-between items-center">
                <div class="font-semibold bg-[#fdfdfd] text-black dark:bg-[#0a0a0a] dark:text-white text-lg">Reginald Ojunga</div>

                <ThemeToggle />
            </div>
        </header>
    }
}
