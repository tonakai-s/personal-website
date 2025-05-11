use leptos::prelude::*;
use crate::components::icons::GithubIcon;

#[component]
pub fn Portolio() -> impl IntoView {
    view! {
        <section class="w-full sm:w-sm lg:w-lg mx-auto mt-6 px-4 sm:px-0">
            <h3 class="text-2xl lg:text-3xl font-bold">"/home/portfolio"</h3>
            <PortolioItem
                name="http-server"
                description="A http server made with Rust following the CodeCrafters website."
                gh_link="https://github.com/tonakai-s/codecrafters-http-server-rust"
            />
            <PortolioItem
                name="this-page"
                description="This page source code."
                gh_link="https://github.com/tonakai-s/personal-website"
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
        <div class="pl-6 mt-2">
            <div class="flex items-center">
                <p class="text-xl lg:text-2xl font-bold text-green-blue mr-2">"/"{name}</p>
                <GithubIcon
                    url={gh_link.into()}
                />
            </div>
            <p class="lg:text-lg">{description}</p>
        </div>
    }
}
