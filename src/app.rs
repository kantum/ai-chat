use leptos::*;
use leptos_meta::*;

mod components;

use crate::api::converse;
use crate::app::components::chat_area::ChatArea;
use crate::app::components::menu::Menu;
use crate::app::components::type_area::TypeArea;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversation, set_conversation) =
        create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };

        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        converse(cx, conversation.get())
    });

    create_effect(cx, move |_| {
        if send.input().get().is_some() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/ai-chat.css"/>
        <Title text="AI Chat"/>
        <div class="flex h-screen flex-col bg-slate-800">
            <Menu/>
            <ChatArea conversation/>
            <TypeArea send/>
        </div>
    }
}
