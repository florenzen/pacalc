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
) -> impl IntoView {
    view! {
        <div>
            <label>
                "Splits (m): "
                <input on:input=move |ev| {
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
                } />
            </label>
        </div>
    }
}
