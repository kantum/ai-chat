use leptos::html::Input;
use leptos::*;

#[component]
pub fn TypeArea(
    cx: Scope,
    send: Action<String, Result<String, ServerFnError>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    view! {cx,
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = input_ref.get().expect("input to exist");
            send.dispatch(input.value());
            input.set_value("");
        } class="space-x-3 h-24 w-full fixed border border-blue-200 bottom-0 flex justify-center items-center p-5 bg-blue-100">
            <input type="text" placeholder="Enter your prompt" node_ref=input_ref class="w-5/6 p-4 border border-blue-200 rounded-lg focus:outline-none focus:ring focus:border-blue-200" />
            <input type="submit" class="h-full p-4 bg-blue-500 text-white rounded-lg cursor-pointer"/>
        </form>
    }
}
