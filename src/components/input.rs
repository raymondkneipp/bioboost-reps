use leptos::prelude::*;

#[component]
pub fn NumberInput(
    value: ReadSignal<f64>,
    set_value: WriteSignal<f64>,
    step: f64,
    #[prop(optional, default = f64::NEG_INFINITY)] min: f64,
    #[prop(optional, default = f64::INFINITY)] max: f64,
) -> impl IntoView {
    view! {
        <input
            class="bg-slate-800 text-center h-12 rounded-full focus:outline-none focus:ring focus:ring-indigo-400 text-indigo-200"
            type="number"
            step=step
            min=min
            max=max
            value=move || value().to_string()
            on:input=move |ev| {
                if let Ok(parsed) = event_target_value(&ev).parse::<f64>() {
                    set_value.set(parsed);
                } else {
                    set_value.set(0.0);
                }
            }
        />
    }
}
