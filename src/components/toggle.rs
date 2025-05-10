use leptos::prelude::*;

#[component]
pub fn ToggleDarkLight() -> impl IntoView {
    let (dark_mode, set_dark_mode) = signal(true);

    Effect::new(move |_| {
        let class = if dark_mode.get() { "dark" } else { "" };
        document()
            .get_elements_by_tag_name("html")
            .get_with_index(0)
            .unwrap()
            .set_class_name(class);
    });

    view! {
            <img
                on:click=move |_| set_dark_mode.set(!dark_mode.get())
                class="cursor-pointer hidden dark:block"
                src="/images/sun-icon.svg"
                alt="Toggle light"
                width="30px"
            />
            <img
                on:click=move |_| set_dark_mode.set(!dark_mode.get())
                class="cursor-pointer block dark:hidden"
                src="/images/moon-icon.svg"
                alt="Toggle dark"
                width="30px"
            />

    }
}
