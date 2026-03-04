use leptos::prelude::*;
use portfolio::App;
// use portfolio::components::about::About;
// use portfolio::components::contacts::Contacts;
fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
            // <About />
            // <Contacts />
        }
    })
}
