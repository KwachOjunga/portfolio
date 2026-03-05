use leptos::prelude::*;
use leptos_theme::Theme;
use leptos_theme::use_theme;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme_signal = use_theme();
    let theme_buttons: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::System];
    view! {
        <div class="space-y-20">
            <div class="space-y-2">
                <p class="">
                    Current theme: {move || theme_signal.get().to_string()}
                </p>

                <div class="flex space-x-4">
                    <p>
                    "Select theme: "
                    </p>
                    {theme_buttons
                        .into_iter()
                        .map(|theme| {
                            view! {
                                <button
                                    on:click=move |_| {
                                    theme_signal.set(theme);
                                }>
                                    <p>{theme.to_string()}</p>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
            </div>
    }
}
