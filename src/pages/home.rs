// use crate::components::counter_btn::Button;
use crate::components::about::About;
use crate::components::contacts::Contacts;
use leptos::prelude::*;
// use leptos_theme::ThemeProvider;
// use leptos_theme::types::Theme;
// use leptos_theme::use_theme;
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        // <ThemeProvider>
        <main class="font-robotomono h-screen flex flex-col items-center justify-around bg-[#fdfdfd] dark:bg-[#0a0a0a] dark:text-white w-full">
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
        // <ThemeButton />
        <About />
        <Contacts />
            // <div class="container">

            //     <picture>
            //         <source
            //             srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
            //             media="(prefers-color-scheme: dark)"
            //         />
            //         <img
            //             src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
            //             alt="Leptos Logo"
            //             height="200"
            //             width="400"
            //         />
            //     </picture>

            //     <h1>"Welcome to Leptos"</h1>

            //     // <div class="buttons">
            //     //     <Button />
            //     //     <Button increment=5 />
            //     // </div>

            // </div>
        </ErrorBoundary>
        </main>
        // </ThemeProvider>
    }
}

// #[component]
// fn ThemeButton() -> impl IntoView {
//     // const CRATE_CSS: &'static str = "text-teal-800 dark:text-teal-300";
//     // const THEME_CSS: &'static str = "text-indigo-800 dark:text-indigo-300";
//     // const LINK_CSS: &'static str =
//     //     "hover:underline underline-offset-4 w-fit decoration-teal-800 dark:decoration-teal-300";
//     // const REPO_LINK: &'static str = "https://github.com/friendlymatthew/leptos-theme";

//     // let footer_links: [(&'static str, String); 2] = [
//     //     ("Contribute", format!("{}", REPO_LINK)),
//     //     ("Leave an issue", format!("{}/issues/new", REPO_LINK)),
//     // ];

//     let theme_buttons: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::System];

//     // 2. retrieve the theme_signal global state
//     let theme_signal = use_theme();

//     view! {
//                 // <main class="font-robotomono h-screen flex flex-col items-center justify-around bg-[#fdfdfd] dark:bg-[#0a0a0a] dark:text-white w-full">
//                     <div class="space-y-24">
//                         <div>
//                             // <a class=LINK_CSS href=REPO_LINK target="_blank" rel="noreferrer">
//                             //     <p class="text-lg lg:text-3xl font-semibold">leptos-theme</p>
//                             // </a>
//                             <br/>
//                             // <div class="space-y-1">
//                             //     <p>Perfect leptos dark mode in 2 lines of code.</p>
//                             //     <em class="text-sm">
//                             //         "Ok a little more than 2 lines if you want to toggle between themes."
//                             //     </em>
//                             // </div>
//                         </div>
//                         <div class="space-y-20">
//                             <div class="space-y-2">
//                                 <p class="">
//                                     Current theme: {move || theme_signal.get().to_string()}
//                                 </p>

//                                 <div class="flex space-x-4">
//                                     <p>
//                                     "Select theme: "
//                                     </p>
//                                     {theme_buttons
//                                         .into_iter()
//                                         .map(|theme| {
//                                             view! {
//                                                 <button
//                                                     on:click=move |_| {
//                                                     theme_signal.set(theme);
//                                                 }>
//                                                     <p>{theme.to_string()}</p>
//                                                 </button>
//                                             }
//                                         })
//                                         .collect::<Vec<_>>()}
//                                 </div>
//                             </div>
//     //                         <div class="">
//     //                             <div class="p-4 bg-[#f0f0f0] dark:bg-[#131313] hover:cursor-text">
//     //                             <pre>
//     //                                 <span>
//     //                                     "use "
//     //                                     <a href="https://crates.io/crates/leptos_theme/0.1.1" class=LINK_CSS target="_blank" rel="noreferrer">
//     //                                         <span class=CRATE_CSS>"leptos_theme"</span>
//     //                                     </a>"::theme::"<span class=THEME_CSS>"ThemeProvider"</span>";"
//     //                                 </span>
//     //                                 <br /> <br />
//     //                                 <span class=THEME_CSS>
//     //                                     "<ThemeProvider>"
//     //                                 </span>
//     //                                 <br/>
//     // "   <Router>
//     //         <Routes>
//     //             <Route path=\"\" view=HomePage/>
//     //         </Routes>
//     //     </Router>"<br/>
//     //     <span class=THEME_CSS>
//     //         "</ThemeProvider>"
//     //     </span>
//     //                             </pre>
//     //                             </div>
//     //                         </div>
//                         </div>
//                     </div>

//                     // <div class="flex justify-center space-x-8">
//                     //     {
//                     //         footer_links
//                     //             .into_iter()
//                     //             .map(|(text, link)| {
//                     //         view! {
//                     //             <a
//                     //                 class=LINK_CSS
//                     //                 href=link
//                     //                 target="_blank"
//                     //                 rel="noreferrer"
//                     //             >
//                     //                 {text}
//                     //             </a>
//                     //         }
//                     //     }).collect::<Vec<_>>()}
//                     // </div>

//                 // </main>
//             }
// }
