#![recursion_limit = "512"]

mod agents;
mod app;
mod components;
mod constants;
mod html;
mod pages;
mod routes;
mod services;
mod state;

use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator when the `wee_alloc` feature is
// enabled.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    // Provide better error messages during panics when the
    // `console_error_panic_hook` feature is enabled
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{}] {}", record.target(), message))
        })
        .chain(fern::Output::call(console_log::log))
        .level(log::LevelFilter::Info)
        .level_for("cards_client_rs", log::LevelFilter::Trace)
        .apply()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    yew::start_app::<app::App>();
    Ok(())
}
