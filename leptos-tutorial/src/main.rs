use leptos::*;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component()]
fn App(cx: Scope) -> impl IntoView {
    // here we create a reactive signal
    // and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    let (count, set_count) = create_signal(cx, 0);

    let double_count = move || count() * 2;

    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! { cx,
        // on:click will run whenever the `click` event fires
        // every event handler is defined as `on:{eventname}`

        // we're able to move `set_count` into the closure
        // because signals are Copy and 'static
        <button
            class:red = move || count() % 2 == 1
            on:click =
                move |_| {
                    set_count.update(|n| *n += 1);
            }
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            "Click me: "
        </button>
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so we can remove the wrapping closure
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // it simply gets the value of count once
            {count()}
        </p>
        <progress
            max="50"
            // signals are functions, so this <=> `move || count.get()`
            value=double_count
        />
        <p>
            "Double count: "
            {double_count}
        </p>
    }
}

#[component()]
fn ProgressBar(cx: Scope, progress: ReadSignal<i32>) -> impl IntoView {
    view! { cx,
        <progress
            max = "50"
            value = progress
        />
    }
}

// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}
