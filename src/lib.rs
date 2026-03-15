mod translate; use translate::translate;

slint::include_modules!();

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() -> Result<(), slint::PlatformError> {
    // Nicer panic messages in the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let ui = Main::new()?;
    let ui_handle = ui.as_weak();

    ui.on_translate(move |input| {
        let output = translate(&input);
        if let Some(ui) = ui_handle.upgrade() {
            ui.invoke_set_output_text(output.into());
        }
    });

    ui.run()
}