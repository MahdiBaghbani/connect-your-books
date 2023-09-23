use seed::*;

pub mod components;
pub mod constants;
pub mod inits;
pub mod models;
pub mod pages;
pub mod updates;
pub mod urls;
pub mod views;

// (This function is invoked by `init` function in `index.html`.)
// #[wasm_bindgen(start)]
// above line is not necessary since trunk v0.17, remained here as a reference.
pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", inits::init, updates::update, views::view);
}
