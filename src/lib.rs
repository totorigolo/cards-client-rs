#![recursion_limit = "512"]

mod app;

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

    web_logger::init();
    yew::start_app::<app::App>();
    Ok(())
}
