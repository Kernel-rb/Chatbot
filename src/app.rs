use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx:Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (coversation , set_conversation) = create_signal(cx, Conversation::new());

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Kernel-rb"/>
        <ChatArea/>
        <TypeArea/>
    }
}
