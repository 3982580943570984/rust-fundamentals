use leptos::*;

/// A list of counters
#[component()]
pub fn StaticList(
    cx: Scope,
    /// Amount of counters in list
    length: usize,
) -> impl IntoView {
    let counters = (1..=length).map(|index| create_signal(cx, index));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button on:click = move |_| set_count.update(|n| *n += 1) >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view(cx);

    view! { cx,
        <ul>{counter_buttons}</ul>
    }
}
