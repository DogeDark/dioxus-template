use dioxus::prelude::*;
use dioxus_logger::tracing::info;

const CALCULATOR_CSS: Asset = asset!("/assets/styling/calculator.css");

/// Calculator component that calls to our fullstack server to perform calculations.
#[component]
pub fn Calculator() -> Element {
    let mut first_number = use_signal(|| 1);
    let mut second_number = use_signal(|| 2);
    let mut result: Signal<Option<i32>> = use_signal(|| None);

    rsx! {
        document::Link { rel: "stylesheet", href: CALCULATOR_CSS }

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
                        if let Ok(data) = server::add_numbers(first_number(), second_number()).await {
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