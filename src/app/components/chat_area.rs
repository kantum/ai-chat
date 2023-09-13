use crate::model::conversation::Conversation;
use leptos::{html::Div, *};

const USER_MESSAGE_CLASS: &str =
    "max-w-md flex items-center self-end rounded-xl rounded-br bg-blue-500 py-2 px-3 text-white";
const MODEL_MESSAGE_CLASS: &str =
    "max-w-md flex items-center self-start rounded-xl rounded-bl bg-slate-700 py-2 px-3 text-white";

#[component]
pub fn ChatArea(
    cx: Scope,
    conversation: ReadSignal<Conversation>,
) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        cx,
        <div class="flex-grow overflow-y-auto bg-slate-800 no-scrollbar" node_ref=chat_div_ref>
        { move || conversation.get().messages.iter().map(move |message| {
            let class_str = if message.user { USER_MESSAGE_CLASS } else { MODEL_MESSAGE_CLASS };
            view! {cx,
                <div class="flex flex-col space-y-2 p-4">
                <div class={class_str}>
                {message.text.clone()}
                </div>
                </div>
            }
        }).collect::<Vec<_>>()
        }
        </div>
    }
}
