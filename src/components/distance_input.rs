use crate::form_state::FormState;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn DistanceInput(
    id: usize,
    distance_get: ReadSignal<usize>,
    distance_set: WriteSignal<usize>,
    error_message_set: WriteSignal<String>,
    set_form_states: WriteSignal<HashMap<usize, FormState>>,
) -> impl IntoView {
    view! {
        <div>
            <label>
                "Distance (m): "
                <input on:input=move |ev| {
                    let input_value = event_target_value(&ev);
                    if input_value.trim().is_empty() {
                        distance_set.set(0);
                        error_message_set.set(String::new());
                    } else {
                        match input_value.parse::<usize>() {
                            Ok(value) => {
                                if value == 0 {
                                    error_message_set
                                        .set("Distance must be greater than 0".to_string());
                                } else {
                                    distance_set.set(value);
                                    error_message_set.set(String::new());
                                }
                            }
                            Err(_) => {
                                error_message_set
                                    .set("Distance must be a positive number".to_string());
                            }
                        }
                    }
                    set_form_states
                        .update(|states| {
                            if let Some(state) = states.get_mut(&id) {
                                state.distance = distance_get.get();
                            }
                        });
                } />
            </label>
        </div>
    }
}
