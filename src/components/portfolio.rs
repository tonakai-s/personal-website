use leptos::prelude::*;
use crate::components::icons::GithubIcon;

#[component]
pub fn Portolio() -> impl IntoView {
    view! {
        <section class="w-full mx-auto mt-6 px-4">
            <h3 class="text-2xl font-bold">"/home/portfolio"</h3>
            <PortolioItem
                name="http-server"
                description="A http server made with Rust following the CodeCrafters website."
                gh_link="https://github.com/tonakai-s/codecrafters-http-server-rust"
            />
        </section>
    }
}

#[component]
fn PortolioItem(
    name: &'static str,
    description: &'static str,
    gh_link: &'static str,
) -> impl IntoView {
    view! {
        <div class="pl-6 mt-1">
            <div class="flex items-center">
                <p class="text-xl font-bold text-green-blue mr-2">"/"{name}</p>
                <GithubIcon
                    url={gh_link.into()}
                />
            </div>
            <p>{description}</p>
        </div>
    }
}
