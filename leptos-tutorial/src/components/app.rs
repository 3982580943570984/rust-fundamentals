use crate::components::static_list::StaticList;
use leptos::*;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component()]
pub fn App(cx: Scope) -> impl IntoView {
    // here we create a reactive signal
    // and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    // let (count, set_count) = create_signal(cx, 0);
    //
    // let double_count = move || count() * 2;
    //
    // // the `view` macro is how we define the user interface
    // // it uses an HTML-like format that can accept certain Rust values
    // view! { cx,
    //     // on:click will run whenever the `click` event fires
    //     // every event handler is defined as `on:{eventname}`
    //
    //     // we're able to move `set_count` into the closure
    //     // because signals are Copy and 'static
    //     <button
    //         class:red = move || count() % 2 == 1
    //         on:click =
    //             move |_| {
    //                 set_count.update(|n| *n += 1);
    //         }
    //     >
    //         // text nodes in RSX should be wrapped in quotes,
    //         // like a normal Rust string
    //         "Click me: "
    //     </button>
    //     <p>
    //         <strong>"Reactive: "</strong>
    //         // you can insert Rust expressions as values in the DOM
    //         // by wrapping them in curly braces
    //         // if you pass in a function, it will reactively update
    //         {move || count.get()}
    //     </p>
    //     <p>
    //         <strong>"Reactive shorthand: "</strong>
    //         // signals are functions, so we can remove the wrapping closure
    //         {count}
    //     </p>
    //     <p>
    //         <strong>"Not reactive: "</strong>
    //         // it simply gets the value of count once
    //         {count()}
    //     </p>
    //     <p>
    //         "ProgressBar with count"
    //         <ProgressBar
    //             progress = count
    //         />
    //     </p>
    //     <p>
    //         "ProgressBar with double_count"
    //         <ProgressBar
    //             // signals are functions, so this <=> `move || count.get()`
    //             progress = Signal::derive(cx, double_count)
    //         />
    //     </p>
    //     <p>
    //         "Double count: "
    //         {double_count}
    //     </p>
    // }

    view! {cx,
        <p>"List of counters"</p>
        <StaticList length = 3/>
    }
}
