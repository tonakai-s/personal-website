use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="absolute w-full bg-blue-black dark:bg-iced-white grid justify-center items-center h-[50px] bottom-0">
            <div>
                <p class="block text-iced-white dark:text-blue-black py-3">"EOF"</p>
            </div>
        </footer>
    }
}
