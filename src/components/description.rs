use leptos::prelude::*;

#[component]
pub fn Description() -> impl IntoView {
    view! {
        <section class="w-full mx-auto mt-6 px-4">
            <p>"Hi, my name is Renan! I'm a guy who is very curious about this hard place that is the computer science field."</p>
            <p>"I love games, music, art, and of course programming."</p>
            <br />
            <p>"Learning Rust to be my main stack, but have experience in other languages like PHP, JS and TS (Most with Node),
            Python and a little bit of notion with Java (Spring Boot)."</p>
            <br />
            <p>"This personal site was made using "<a href="https://leptos.dev/" class="text-green-blue font-bold" target="_blank">"Leptos"</a> " with " <a href="https://tailwindcss.com" class="text-green-blue font-bold" target="_blank">"TailwindCSS"</a>" and I pretend
            to implement some other features to stylize it while I learn more about the framework."</p>
        </section>
    }
}
