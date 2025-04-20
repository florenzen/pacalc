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
) -> impl IntoView {
    view! {
        <div>
            <label>
                "Pace (mm:ss/km): "
                <input
                    type="text"
                    on:input=move |ev| {
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
                    }
                />
            </label>
        </div>
    }
}
