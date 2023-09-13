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
        } class="flex items-center p-4 bg-slate-700">
            <input type="text" placeholder="Enter your prompt" node_ref=input_ref class="w-full bg-slate-800 rounded-lg border border-slate-700 text-white focus:outline-none focus:ring focus:border-slate-700 px-4 py-2" />
            <input type="submit" class="ml-2 rounded-lg bg-blue-500 px-4 py-2 text-white cursor-pointer"/>
        </form>
    }
}
