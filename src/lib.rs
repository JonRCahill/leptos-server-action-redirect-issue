pub mod app;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();
      console_log::init_with_level(log::Level::Debug).expect("setup logging");

      leptos::mount_to_body(move || {
          view! { <App/> }
      });
    }
}
}
