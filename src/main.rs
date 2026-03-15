slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = Main::new()?;

    ui.run()
}
