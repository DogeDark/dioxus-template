//! This crate contains all fullstack server functions.
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

/// Add two numbers together on the server.
#[server(AddNumbers)]
pub async fn add_numbers(first: i32, last: i32) -> Result<i32, ServerFnError> {
    info!("Server is calculating `{} + {}`", first, last);
    Ok(first + last)
}