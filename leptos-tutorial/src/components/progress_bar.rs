use leptos::*;

/// Show progress toward a goal.
#[component()]
pub fn ProgressBar(
    cx: Scope,
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
        <progress
            max = max
            value = progress
        />
    }
}
