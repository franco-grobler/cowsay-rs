//! Perform global constructor initialization.
//!
//! This function is conditionally compiled for WASM targets. It explicitly calls
//! `__wasm_call_ctors()` to ensure that global constructors, particularly those
//! registered by crates like `inventory`, are executed. This is necessary because
//! WASM environments might not automatically call these constructors as native
//! environments do.
use wasm_bindgen::prelude::wasm_bindgen;

use cowsay::cows;

pub mod options;

#[cfg(target_family = "wasm")]
#[expect(unsafe_code)]
/// Initialize global constructors for WASM targets.
pub fn before_main() {
    unsafe extern "C" {
        fn __wasm_call_ctors();
    }

    // Salsa uses the `inventory` crate, which registers global constructors that may need to be
    // called explicitly on WASM. See <https://github.com/dtolnay/inventory/blob/master/src/lib.rs#L105>
    // for details.
    unsafe {
        __wasm_call_ctors();
    }
}

#[cfg(not(target_family = "wasm"))]
/// Do nothing on non-WASM targets.
pub const fn before_main() {}

#[wasm_bindgen(start)]
/// Main entry point for the WebAssembly module.
///
/// This function is automatically called when the WASM module is instantiated.
/// It performs necessary setup, including:
/// - Calling `before_main` for global constructor initialization.
/// - Setting up a panic hook for better error messages in the browser console
///   when the `console_error_panic_hook` feature is enabled.
/// - Initialising the `console_log` logger to output messages to the browser's
///   developer console.
///
/// # Panics
/// WASM could not be initialised if the logger fails to initialize.
pub fn run() {
    use log::Level;

    before_main();

    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    console_log::init_with_level(Level::Debug)
        .expect("Initializing logger went wrong.");
}

#[wasm_bindgen(js_name = listCows)]
/// Return a list of all selectable cows
pub fn list_cows() -> Vec<String> {
    cows::list_cows()
}
