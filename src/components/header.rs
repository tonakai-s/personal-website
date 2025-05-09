use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            class="w-full grid justify-center mt-6"
        >
            <div class="container-sm grid border-solid border-1 border-blue-black w-xl justify-center">
                <div>
                    <h1 class="text-2xl font-bold">tonakai-s</h1>
                    <h2 class="text-lg">(トナカイ-ス)</h2>
                    <p>A software developer</p>
                </div>
            </div>
        </header>
    }
}