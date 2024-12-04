use leptos::prelude::*;

#[component]
pub fn Button(
    children: ChildrenFn,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
) -> impl IntoView {
    view! {
        <button
            class="bg-slate-800 text-indigo-200 text-2xl rounded-full w-12 h-12 flex items-center justify-center focus:outline-none focus:ring focus:ring-indigo-400"
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}
