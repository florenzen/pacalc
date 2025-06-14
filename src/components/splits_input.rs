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

use crate::form_state::FormState;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn SplitsInput(
    id: usize,
    splits_get: ReadSignal<usize>,
    splits_set: WriteSignal<usize>,
    error_message_set: WriteSignal<String>,
    set_form_states: WriteSignal<HashMap<usize, FormState>>,
    #[prop(default = false)] is_grid: bool,
) -> impl IntoView {
    let handle_input = move |ev| {
        let input_value = event_target_value(&ev);
        if input_value.trim().is_empty() {
            splits_set.set(0);
            error_message_set.set(String::new());
        } else {
            match input_value.parse::<usize>() {
                Ok(value) => {
                    if value == 0 {
                        error_message_set
                            .set("Splits must be greater than 0".to_string());
                    } else {
                        splits_set.set(value);
                        error_message_set.set(String::new());
                    }
                }
                Err(_) => {
                    error_message_set
                        .set("Splits must be a positive number".to_string());
                }
            }
        }
        set_form_states
            .update(|states| {
                if let Some(state) = states.get_mut(&id) {
                    state.splits = splits_get.get();
                }
            });
    };

    if is_grid {
        view! {
            <div class="flex items-center">
                <span class="whitespace-nowrap">"Splits (m):"</span>
            </div>
            <input
                type="number"
                inputmode="numeric"
                pattern="[0-9]*"
                class="w-32 px-2 py-1 rounded"
                on:input=handle_input
            />
        }.into_any()
    } else {
        view! {
            <div>
                <label>
                    "Splits (m): "
                    <input 
                        type="number" 
                        inputmode="numeric"
                        pattern="[0-9]*"
                        on:input=handle_input 
                    />
                </label>
            </div>
        }.into_any()
    }
}
