// Copyright (c) 2024 Florian Lorenzen

// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without
// restriction, including without limitation the rights to use,
// copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.

use crate::colors;
use crate::components::{
    DeleteButton, DistanceInput, ErrorMessage, LabelInput, PaceInput, SplitToggle, SplitsInput,
    SplitsList, TotalDuration,
};
use crate::form_state::FormState;
use leptos::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

#[component]
pub fn PaceCalculatorForm(
    id: usize,
    on_delete: Option<Callback<usize>>,
    form_state: Memo<FormState>,
    set_form_states: WriteSignal<HashMap<usize, FormState>>,
) -> impl IntoView {
    let (splits_get, splits_set) = signal(form_state.get().splits);
    let (distance_get, distance_set) = signal(form_state.get().distance);
    let (show_splits_get, show_splits_set) = signal(form_state.get().show_splits);
    let (pace_get, pace_set) = signal(form_state.get().pace);
    let (error_message_get, error_message_set) = signal(String::new());
    let (label_get, label_set) = signal(form_state.get().label);
    let total_duration = Memo::new(move |_| {
        let pace = pace_get.get();
        let distance = distance_get.get();
        let seconds = if pace > Duration::ZERO && distance > 0 {
            Some((distance as f64 / 1000.0) * pace.as_secs_f64())
        } else {
            None
        };
        seconds.map(Duration::from_secs_f64)
    });

    view! {
        <div
            class="border border-solid border-gray-300 p-4 mb-5 rounded-md text-white"
            style=format!("background-color: {}", colors::BLUE1)
        >
            <div class="flex flex-col md:flex-row flex-wrap gap-5 mb-4 items-start justify-between">
                {/* Label component - only shown on small screens */}
                <div class="w-full flex justify-end md:hidden">
                    <LabelInput
                        id=id
                        label_get=label_get
                        label_set=label_set
                        set_form_states=set_form_states.clone()
                    />
                    {move || match on_delete.clone() {
                        Some(callback) => {
                            view! { <DeleteButton id=id callback=callback /> }.into_any()
                        }
                        None => view! { <></> }.into_any(),
                    }}
                </div>
                
                <div class="flex flex-wrap gap-5 items-center flex-1">
                    <PaceInput
                        id=id
                        pace_get=pace_get
                        pace_set=pace_set
                        error_message_set=error_message_set
                        set_form_states=set_form_states.clone()
                    />
                    <SplitsInput
                        id=id
                        splits_get=splits_get
                        splits_set=splits_set
                        error_message_set=error_message_set
                        set_form_states=set_form_states.clone()
                    />
                    <DistanceInput
                        id=id
                        distance_get=distance_get
                        distance_set=distance_set
                        error_message_set=error_message_set
                        set_form_states=set_form_states.clone()
                    />
                    <TotalDuration total_duration=total_duration />
                </div>
                
                {/* Label component - only shown on medium and larger screens */}
                <div class="hidden md:flex md:w-auto items-center justify-end">
                    <LabelInput
                        id=id
                        label_get=label_get
                        label_set=label_set
                        set_form_states=set_form_states.clone()
                    />
                    {move || match on_delete.clone() {
                        Some(callback) => {
                            view! { <DeleteButton id=id callback=callback /> }.into_any()
                        }
                        None => view! { <></> }.into_any(),
                    }}
                </div>
            </div>
            <ErrorMessage error_message_get=error_message_get />
            <div class="flex items-baseline">
                <SplitToggle
                    id=id
                    show_splits_get=show_splits_get
                    show_splits_set=show_splits_set
                    set_form_states=set_form_states.clone()
                />
                <SplitsList
                    pace_get=pace_get
                    distance_get=distance_get
                    splits_get=splits_get
                    show_splits_get=show_splits_get
                />
            </div>
        </div>
    }
}
