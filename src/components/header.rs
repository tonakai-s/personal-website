use leptos::prelude::*;

use crate::components::{
    icons::{GithubIcon, Icon},
    toggle::ToggleDarkLight,
};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            class="w-full grid justify-center"
        >
            <div class="grid border-solid border-1 border-blue-black dark:border-iced-white justify-center py-2 px-12 sm:w-sm lg:w-lg">
                <h1 class="text-2xl lg:text-3xl font-bold">"tonakai-s"</h1>
                <h2 class="text-lg">"(トナカイ-ス)"</h2>
                <p class="lg:text-lg">"A software developer"</p>
            </div>

            <div class="grid grid-cols-2 items-center mt-3">
                <div>
                    <Nav />
                </div>
                <div class="grid justify-end">
                    <ToggleDarkLight />
                </div>
            </div>
        </header>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <ul class="flex gap-3">
                <li class="w-fit">
                    <GithubIcon
                        width="28px".into()
                    />
                </li>
                <li class="w-fit">
                    <Icon
                        alt="Linkedin Icon".into()
                        light_src="images/linkedin.svg".into()
                        dark_src="images/linkedin-white.svg".into()
                        url=Some("https://br.linkedin.com/in/renandrnls".into())
                        width="28px".into()
                    />
                </li>
                <li class="w-fit">
                    <Icon
                        alt="X Icon".into()
                        light_src="images/x.svg".into()
                        dark_src="images/x-white.svg".into()
                        url=Some("https://x.com/0xRen4s".into())
                        width="28px".into()
                    />
                </li>
            </ul>
        </nav>
    }
}
