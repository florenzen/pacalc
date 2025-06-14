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
use std::time::Duration;

fn parse_duration(s: &str) -> Result<Duration, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid format".to_string());
    }
    let minutes: u64 = parts[0]
        .parse()
        .map_err(|_| "Invalid minutes".to_string())?;
    let seconds: u64 = parts[1]
        .parse()
        .map_err(|_| "Invalid seconds".to_string())?;
    Ok(Duration::from_secs(minutes * 60 + seconds))
}

#[component]
pub fn PaceInput(
    id: usize,
    pace_get: ReadSignal<Duration>,
    pace_set: WriteSignal<Duration>,
    error_message_set: WriteSignal<String>,
    set_form_states: WriteSignal<HashMap<usize, FormState>>,
    #[prop(default = false)] is_grid: bool,
) -> impl IntoView {
    let handle_input = move |ev| {
        let pace_str = event_target_value(&ev);
        if pace_str.trim().is_empty() {
            pace_set.set(Duration::ZERO);
            error_message_set.set(String::new());
        } else {
            match parse_duration(&pace_str) {
                Ok(duration) => {
                    pace_set.set(duration);
                    error_message_set.set(String::new());
                }
                Err(err) => {
                    error_message_set.set(format!("Pace error: {}", err));
                }
            }
        }
        set_form_states
            .update(|states| {
                if let Some(state) = states.get_mut(&id) {
                    state.pace = pace_get.get();
                }
            });
    };

    if is_grid {
        view! {
            <div class="w-full grid grid-cols-2 gap-x-2">
                <div class="flex justify-end items-center">
                    <span class="whitespace-nowrap">"Pace (mm:ss/km):"</span>
                </div>
                <input
                    type="text"
                    class="w-full px-2 py-1 rounded"
                    on:input=handle_input
                />
            </div>
        }.into_any()
    } else {
        view! {
            <div>
                <label>
                    "Pace (mm:ss/km): "
                    <input
                        type="text"
                        on:input=handle_input
                    />
                </label>
            </div>
        }.into_any()
    }
}
