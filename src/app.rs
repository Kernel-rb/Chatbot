use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::model::conversation::Conversation;

#[component]
pub fn App(cx:Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (coversation , set_conversation) = create_signal(cx, Conversation::new());
    let send = create_action(cx , move |new_msg : &String|{
        
    });
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Kernel-rb"/>
                {coversation.get()}
        <ChatArea/>
        <TypeArea/>
    }
}
