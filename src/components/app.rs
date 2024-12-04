use crate::{one_rep_max, plates, weight_conversion};

use super::button::Button;
use super::input::NumberInput;
use super::select::Select;

use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (weight, set_weight) = signal(225.0);
    let (reps, set_reps) = signal(5.0);
    let (unit, set_unit) = signal("pounds".to_string());
    let (formula, set_formula) = signal("best".to_string());

    let weight_unit = move || {
        weight_conversion::WeightUnit::new(
            weight.get(),
            match unit.get().as_str() {
                "pounds" => weight_conversion::WeightUnitType::Pounds,
                "kilograms" => weight_conversion::WeightUnitType::Kilograms,
                _ => panic!("Invalid unit"),
            },
        )
    };

    let one_rm = move || match formula.get().as_str() {
        "best" => one_rep_max::calc(weight_unit(), reps.get() as u8),
        "epley" => one_rep_max::calc_with_formula(
            weight_unit(),
            reps.get() as u8,
            one_rep_max::Formula::Epley,
        ),
        "brzychi" => one_rep_max::calc_with_formula(
            weight_unit(),
            reps.get() as u8,
            one_rep_max::Formula::Brzycki,
        ),
        "mcglothin" => one_rep_max::calc_with_formula(
            weight_unit(),
            reps.get() as u8,
            one_rep_max::Formula::McGlothin,
        ),
        "kelley" => one_rep_max::calc_with_formula(
            weight_unit(),
            reps.get() as u8,
            one_rep_max::Formula::Kelley,
        ),
        _ => panic!("Invalid formula"),
    };

    let weight_increment = move || match unit.get().as_str() {
        "pounds" => 5.0,
        "kilograms" => 2.5,
        _ => panic!("Invalid unit"),
    };

    let plates_for_weight = move || {
        let w = weight_conversion::WeightUnit::new(
            weight.get(),
            match unit.get().as_str() {
                "pounds" => weight_conversion::WeightUnitType::Pounds,
                "kilograms" => weight_conversion::WeightUnitType::Kilograms,
                _ => panic!("Invalid unit"),
            },
        );
        plates::calculate_plates(w)
    };

    let plates_for_1rm = move || {
        let w = one_rm();
        plates::calculate_plates(w)
    };

    // TODO: When weight changes (effect?) make sure it is a valid increment when swapping units

    view! {
        <div class="grid grid-cols-[auto_1fr_auto] items-center max-w-xs mx-auto gap-x-6">
            <label class="col-span-full text-center mb-0.5 text-sm font-bold text-slate-400">
                Reps
            </label>
            <Button on:click=move |_| *set_reps.write() -= 1.0>"-"</Button>
            <NumberInput step=1.0 set_value=set_reps value=reps min=1.0 max=255.0 />
            <Button on:click=move |_| *set_reps.write() += 1.0>"+"</Button>

            <label class="col-span-full text-center mb-0.5 text-sm font-bold text-slate-400 mt-3">
                Weight
            </label>
            <Button on:click=move |_| *set_weight.write() -= weight_increment()>"-"</Button>
            <NumberInput step=2.5 set_value=set_weight value=weight min=weight_increment() />
            <Button on:click=move |_| *set_weight.write() += weight_increment()>"+"</Button>

            <label class="col-span-full text-center mb-0.5 text-sm font-bold text-slate-400 mt-3">
                Unit
            </label>
            <select
                class="bg-slate-800 h-12 rounded-full text-center col-start-2 focus:outline-none focus:ring focus:ring-indigo-400 text-indigo-200"
                on:change:target=move |ev| {
                    set_unit(ev.target().value().parse().unwrap());
                }
                prop:value=move || unit.get().to_string()
            >

                <option value="pounds">Pounds</option>
                <option value="kilograms">Kilograms</option>
            </select>

            <label class="col-span-full text-center mb-0.5 text-sm font-bold text-slate-400 mt-3">
                Formula
            </label>

            <Select
                value=formula
                set_value=set_formula
                options=vec![
                    "best".to_string(),
                    "epley".to_string(),
                    "brzychi".to_string(),
                    "mcglothin".to_string(),
                    "kelley".to_string(),
                ]
            />
        </div>


        <div class="flex items-center justify-center flex-wrap mt-3">
            <For
                each=move || plates_for_weight()
                key=|state| (state.quantity, state.weight.value as u32)
                let:child
            >
                <WeightPlate p=child />
            </For>
        </div>

        <p class="text-center text-lg font-bold mt-3">"1RM: " {move || one_rm().to_string()}</p>

        <div class="flex items-center justify-center flex-wrap mt-3">
            <For
                each=move || plates_for_1rm()
                key=|state| (state.quantity, state.weight.value as u32)
                let:child
            >
                <WeightPlate p=child />
            </For>
        </div>
    }
}

// use leptos::prelude::*;

#[component]
pub fn WeightPlate(p: plates::Plate) -> impl IntoView {
    // Non-linear scaling function: square root
    let base_size = 30.0; // Minimum size for very small weights
    let size = (base_size + p.weight.value.sqrt() * 15.0).round(); // Adjust multiplier as needed

    let size_px = format!("{}px", size);

    // Assign a color based on the weight
    let color = match p.weight.value {
        w if w >= 45.0 => "bg-blue-800",
        w if w >= 35.0 => "bg-green-600",
        w if w >= 25.0 => "bg-yellow-600",
        w if w >= 10.0 => "bg-orange-500",
        w if w >= 5.0 => "bg-red-500",
        w if w >= 2.5 => "bg-purple-500",
        _ => "bg-gray-500",
    };

    view! {
        <div
            style:width=size_px.clone()
            style:height=size_px.clone()
            class=format!("rounded-full flex items-center justify-center text-xs flex-col first:ml-0 -ml-6 {}", color)
        >
            <span>{p.quantity}</span>
            <span>{p.weight.to_string()}</span>
        </div>
    }
}
