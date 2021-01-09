use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run our program, put all program logic here or in subsequent modules.
#[wasm_bindgen]
pub fn run() {
    set_panic_hook();
    // Program logic here
    println!("Hello, world");
}

/// Improve error messages in the browser when running as WebAssembly.
/// For more details see
/// https://github.com/rustwasm/console_error_panic_hook#readme
fn set_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}
