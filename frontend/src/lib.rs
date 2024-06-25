pub mod app;
pub mod backend_client;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod gnb;
pub mod home_page;
pub mod layout;
pub mod post;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
