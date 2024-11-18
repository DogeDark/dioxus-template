use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

{% if is_router -%}
use ui::Navbar;
{%- else -%}
{% if is_fullstack -%}
use ui::{Hero, Calculator};
{%- else -%}
use ui::Hero;
{%- endif %}
{%- endif %}
{% if is_router -%}
use views::{Blog, Home};
{%- endif %}

{% if is_router %}
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
{% endif %}

const MAIN_CSS: Asset = asset!("/assets/main.css");

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

{% if is_router -%}
/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
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