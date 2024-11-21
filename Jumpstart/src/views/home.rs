use dioxus::prelude::*;
{% if is_fullstack -%}
use crate::components::{Hero, Echo};
{%- else -%}
use crate::components::Hero;
{%- endif %}

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        {% if is_fullstack -%}
        Echo {}
        {%- endif %}
    }
}
