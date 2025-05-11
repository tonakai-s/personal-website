use leptos::prelude::*;

#[component]
pub fn Description() -> impl IntoView {
    view! {
        <section class="w-full mx-auto mt-6 px-4">
            <p>"Hi, my name is Renan! I'm a curious guy diving into the challenging world of computer science."</p>
            <br />
            <p>"I love games, music, art, and of course, programming."</p>
            <p>"I'm also learning Japanese—not because of anime, but because I think the language is really beautiful (so far, just hiragana and katakana)."</p>
            <br />
            <p>"Learning Rust now, but have experience in other languages like PHP, JS and TS (Most with Node),
            Python and a little bit of notion with Java (Spring Boot)."</p>
            <br />
            <p>"What 'tonakai-s' mean? My name is Renan, but my friends call me Renas(portuguese), wich is the plural of reindeer, and reindeer in japanese is tonakai, the '-s' or 'su' is a kind of self plural, just to represent the portuguese version."</p>
            <br />
            <p>"This personal site was made using "<a href="https://leptos.dev/" class="text-green-blue font-bold" target="_blank">"Leptos"</a> " with " <a href="https://tailwindcss.com" class="text-green-blue font-bold" target="_blank">"TailwindCSS"</a>" and I pretend
            to implement some other features to stylize it while I learn more about the framework."</p>
        </section>
    }
}
