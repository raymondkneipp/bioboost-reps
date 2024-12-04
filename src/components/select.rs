use leptos::prelude::*;

#[component]
pub fn Select(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    options: Vec<String>,
) -> impl IntoView {
    let (options_signal, _options_signal) = signal(options);

    view! {
        <select
            class="bg-slate-800 h-12 rounded-full text-center col-start-2 focus:outline-none focus:ring focus:ring-indigo-400 text-indigo-200 capitalize"
            prop:value=move || value().to_string()
            on:change:target=move |ev| {
                set_value.set(ev.target().value());
            }
        >

            <For each=move || options_signal.get() key=|state| state.clone() let:child>
                <option value={child.clone()}>{child.clone()}</option>
            </For>
        </select>
    }
}
