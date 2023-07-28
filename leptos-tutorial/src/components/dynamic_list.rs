use leptos::*;

/// A list of counters that allows to add or remove counters.
#[component()]
pub fn DynamicList(
    cx: Scope,
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    view! { cx,
        <div></div>
    }
}
