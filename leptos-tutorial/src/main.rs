pub mod components;

use crate::components::app::App;
use leptos::*;

// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}
