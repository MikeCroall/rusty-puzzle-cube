mod demo;
mod gui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    demo::run();
    Ok(())
}

/// For non-wasm targets, the binary entry point has been moved to a different file.
/// This start function remains able to run the demo, gui or otherwise, but is
/// primarily here to avoid tooling thinking almost the entire crate is unused.
#[cfg(not(target_arch = "wasm32"))]
pub fn start() {
    demo::run();
}
