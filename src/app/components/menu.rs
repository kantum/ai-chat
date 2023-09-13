use leptos::*;

#[component]
pub fn Menu(
    cx: Scope,
) -> impl IntoView {
    view! {cx,
        <div class="sticky w-full border border-gray-300 top-0 flex justify-center items-center p-3 bg-blue-100">
        <h1 class="text-3xl font-bold">AI Chat</h1>
        </div>
    }
}
