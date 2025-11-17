//! Test WASM API

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::wasm_bindgen_test;

use cowsay_wasm::options;

#[wasm_bindgen_test]
fn it_can_create_default_options() {
    let options = options::Options::default_options();
    options.say("Hello, world!").unwrap();
}
