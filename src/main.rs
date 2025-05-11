use leptos::prelude::*;
use tonakais_dev::components::{description::Description, header::Header, portfolio::Portolio, footer::Footer};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Header />
        <main class="w-full">
            <Description />
            <Portolio />
        </main>
        <Footer />
    }
}
