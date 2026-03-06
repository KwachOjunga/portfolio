// use crate::components::counter_btn::Button;
use crate::components::about::About;
use crate::components::contacts::Contacts;
use crate::components::header::Header;
use crate::components::hero::Hero;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {

        // <main class="font-robotomono min-h-screen flex flex-col items-center justify-around bg-[#fdfdfd] dark:bg-[#0a0a0a] dark:text-white w-full">
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
        <main class="font-robotomono min-h-screen flex flex-col items-center justify-around bg-[#fdfdfd] dark:bg-[#0a0a0a] dark:text-white w-full">
        <Header />
        <Hero />
        <About />
        <Contacts />
        </main>
        </ErrorBoundary>

    }
}
