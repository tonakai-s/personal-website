use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full bg-blue-black dark:bg-iced-white grid justify-center items-center mt-6">
            <div>
                <p class="block text-iced-white dark:text-blue-black py-3">"EOF"</p>
            </div>
        </footer>
    }
}
