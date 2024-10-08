mod app;
mod component;
mod talk;

use browser_panic_hook::{Basic, IntoPanicHook};
use wasm_bindgen::JsValue;

pub fn main() -> Result<(), JsValue> {
    // provide a custom panic hook

    yew::set_custom_panic_hook(Basic.into_panic_hook());

    // console log

    console_log::init().expect("Could not initialize logger");

    // start the application

    yew::Renderer::<app::Application>::new().render();

    // done

    Ok(())
}
