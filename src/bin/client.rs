use leptos::mount_to_body;
use leptos_cloudflare_pages::app::HelloWorld;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    mount_to_body(HelloWorld);
}

pub fn main() {}
