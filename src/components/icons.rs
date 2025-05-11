use leptos::prelude::*;

#[component]
pub fn GithubIcon(
    #[prop(default = "https://github.com/tonakai-s".into())] url: String,
    #[prop(default = "20px".into())] width: String,
) -> impl IntoView {
    view! {
            <a
                href=url
                target="_blank"
            >
                <Icon
                    alt="Github Icon".into()
                    light_src="images/github.svg".into()
                    dark_src="images/github-white.svg".into()
                    width=width
                />
            </a>
    }
}

#[component]
pub fn Icon(
    alt: String,
    light_src: String,
    dark_src: String,
    width: String,
    #[prop(default = None)] url: Option<String>,
) -> impl IntoView {
    let icons = view! {
        <>
            <img
                class="block dark:hidden"
                src={light_src}
                alt={alt.clone()}
                width={width.clone()}
            />
            <img
                class="hidden dark:block"
                src={dark_src}
                alt={alt}
                width={width}
            />
        </>
    };

    if let Some(url) = url {
        view! {
            <a href={url} target="_blank">
                {icons}
            </a>
        }
        .into_any()
    } else {
        view! {
            {icons}
        }
        .into_any()
    }
}
