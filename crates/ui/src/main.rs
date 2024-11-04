use leptos::*;
use ui::home::Home;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).ok();
    mount_to_body(|| view! { <Home /> })
}
