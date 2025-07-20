mod app;
mod smbls;

use crate::app::App;
use leptos::mount::mount_to_body;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    console_error_panic_hook::set_once();

    mount_to_body(App);

    Ok(())
}
