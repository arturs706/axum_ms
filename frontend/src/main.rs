use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(count() + 3)
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}