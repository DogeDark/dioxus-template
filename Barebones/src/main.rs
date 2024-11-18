use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

{% if is_router %}
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
{% endif %}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting app.");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        {% if is_router -%}
        Router::<Route> {}
        {%- else -%}
        Hero {}
        {% if is_fullstack -%}
        Calculator {}
        {%- endif %}
        {%- endif %}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { target: "_blank", href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { target: "_blank", href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

{% if is_router -%}
/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        {% if is_fullstack -%}
        Calculator {}
        {%- endif %}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL paramaters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
{%- endif %}

{% if is_fullstack -%}
/// Calculator component that calls to our fullstack server to perform calculations.
#[component]
fn Calculator() -> Element {
    let mut first_number = use_signal(|| 1);
    let mut second_number = use_signal(|| 2);
    let mut result: Signal<Option<i32>> = use_signal(|| None);

    rsx! {
        div {
            id: "calculator",
            h4 { "ServerFn Calculator" }

            // Top row of inputs
            div {
                id: "calculator-inputs",
                div {
                    class: "calculator-input",
                    label { "First Number: "}
                    input {
                        r#type: "number",
                        value: first_number,
                        oninput: move |event| first_number.set(event.value().parse().expect("input should only be a number")),
                    }
                }
                div {
                    class: "calculator-input",
                    label { "Second Number: "}
                    input {
                        r#type: "number",
                        value: second_number,
                        oninput: move |event| second_number.set(event.value().parse().expect("input should only be a number")),
                    }
                }
            }

            // Bottom row
            div {
                id: "calculator-bottom",

                // Submit button
                button {
                    onclick: move |_| async move {
                        if let Ok(data) = add_numbers(first_number(), second_number()).await {
                            info!("Client received calculated number: {}", data);
                            result.set(Some(data));
                        }
                    },
                    "Add Numbers"
                }

                // Result
                if let Some(result) = result() {
                    p { "Result: {result}" }
                }
            }
        }
    }
}

/// Add two numbers together on the server.
#[server(AddNumbers)]
async fn add_numbers(first: i32, last: i32) -> Result<i32, ServerFnError> {
    info!("Server is calculating `{} + {}`", first, last);
    Ok(first + last)
}
{%- endif %}