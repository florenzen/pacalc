use leptos::prelude::*;
use leptos::*;
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
fn PaceCalculatorForm(id: usize, on_delete: Option<Callback<usize>>) -> impl IntoView {
    let (pace_get, pace_set) = signal(Duration::ZERO);
    let (splits_get, splits_set) = signal(0);
    let (distance_read, distance_write) = signal(0);
    let (show_splits, set_show_splits) = signal(true);
    let (error_message, set_error_message) = signal(String::new());
    let total_duration = Memo::new(move |_| {
        let pace = pace_get.get();
        let distance = distance_read.get();
        let seconds = if pace > Duration::ZERO && distance > 0 {
            Some((distance as f64 / 1000.0) * pace.as_secs_f64())
        } else {
            None
        };
        seconds.map(Duration::from_secs_f64)
    });

    view! {
        <div
            class="calculator-form"
            style="border: 1px solid #ccc; padding: 15px; margin-bottom: 20px; border-radius: 5px;"
        >
            <div style="display: flex; flex-wrap: wrap; gap: 20px; margin-bottom: 15px; align-items: flex-start; justify-content: space-between;">
                <div style="display: flex; flex: 1; flex-wrap: wrap; gap: 20px; align-items: center;">
                    <div>
                        <label>
                            "Pace (mm:ss/km): "
                            <input
                                type="text"
                                on:input=move |ev| {
                                    let pace_str = event_target_value(&ev);
                                    if pace_str.trim().is_empty() {
                                        pace_set.set(Duration::ZERO);
                                        set_error_message.set(String::new());
                                    } else {
                                        match parse_duration(&pace_str) {
                                            Ok(duration) => {
                                                pace_set.set(duration);
                                                set_error_message.set(String::new());
                                            },
                                            Err(err) => {
                                                set_error_message.set(format!("Pace error: {}", err));
                                            }
                                        }
                                    }
                                }
                            />
                        </label>
                    </div>
                    <div>
                        <label>
                            "Splits (m): "
                            <input
                                on:input=move |ev| {
                                    let input_value = event_target_value(&ev);
                                    if input_value.trim().is_empty() {
                                        splits_set.set(0);
                                        set_error_message.set(String::new());
                                    } else {
                                        match input_value.parse::<usize>() {
                                            Ok(value) => {
                                                if value == 0 {
                                                    set_error_message.set("Splits must be greater than 0".to_string());
                                                } else {
                                                    splits_set.set(value);
                                                    set_error_message.set(String::new());
                                                }
                                            },
                                            Err(_) => {
                                                set_error_message.set("Splits error: must be a positive number".to_string());
                                            }
                                        }
                                    }
                                }
                            />
                        </label>
                    </div>
                    <div>
                        <label>
                            "Distance (m): "
                            <input
                                on:input=move |ev| {
                                    let input_value = event_target_value(&ev);
                                    if input_value.trim().is_empty() {
                                        distance_write.set(0);
                                        set_error_message.set(String::new());
                                    } else {
                                        match input_value.parse::<usize>() {
                                            Ok(value) => {
                                                if value == 0 {
                                                    set_error_message.set("Distance must be greater than 0".to_string());
                                                } else {
                                                    distance_write.set(value);
                                                    set_error_message.set(String::new());
                                                }
                                            },
                                            Err(_) => {
                                                set_error_message.set("Distance error: must be a positive number".to_string());
                                            }
                                        }
                                    }
                                }
                            />
                        </label>
                    </div>
                    <div>
                        "Total duration: "
                        {move || {
                            total_duration
                                .get()
                                .map(|time| {
                                    let total_seconds = time.as_secs();
                                    let minutes = total_seconds / 60;
                                    let seconds = total_seconds % 60;
                                    format!("{:02}:{:02}", minutes, seconds)
                                })
                                .unwrap_or_else(|| "N/A".to_string())
                        }}
                    </div>
                </div>
                <div style="margin-left: auto; align-self: flex-start;">
                    {move || match on_delete.clone() {
                        Some(callback) => {
                            view! {
                                <button
                                    on:click=move |_| callback.run(id)
                                    class="delete-btn"
                                    style="background-color: #ff4d4d; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer;"
                                >
                                    "Delete"
                                </button>
                            }
                                .into_any()
                        }
                        None => view! { <></> }.into_any(),
                    }}
                </div>
            </div>
            <div style:color="red" 
                 style:height=move || if error_message.get().is_empty() { "0" } else { "auto" }
                 style:overflow="hidden" 
                 style:margin-bottom=move || if error_message.get().is_empty() { "0" } else { "10px" }>
                {move || error_message.get()}
            </div>
            <div style="display: flex; align-items: center; justify-content: space-between;">
                <span style="display: flex; align-items: center; gap: 5px;">
                    "Splits:"
                    <button 
                        on:click=move |_| set_show_splits.set(!show_splits.get())
                        style="background: transparent; border: none; padding: 5px; border-radius: 3px; cursor: pointer; color: #777;">
                        {move || if show_splits.get() {
                            view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
                                    <path d="M16 8s-3-5.5-8-5.5S0 8 0 8s3 5.5 8 5.5S16 8 16 8zM1.173 8a13.133 13.133 0 0 1 1.66-2.043C4.12 4.668 5.88 3.5 8 3.5c2.12 0 3.879 1.168 5.168 2.457A13.133 13.133 0 0 1 14.828 8c-.058.087-.122.183-.195.288-.335.48-.83 1.12-1.465 1.755C11.879 11.332 10.119 12.5 8 12.5c-2.12 0-3.879-1.168-5.168-2.457A13.134 13.134 0 0 1 1.172 8z"/>
                                    <path d="M8 5.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5zM4.5 8a3.5 3.5 0 1 1 7 0 3.5 3.5 0 0 1-7 0z"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
                                    <path d="M13.359 11.238C15.06 9.72 16 8 16 8s-3-5.5-8-5.5a7.028 7.028 0 0 0-2.79.588l.77.771A5.944 5.944 0 0 1 8 3.5c2.12 0 3.879 1.168 5.168 2.457A13.134 13.134 0 0 1 14.828 8c-.058.087-.122.183-.195.288-.335.48-.83 1.12-1.465 1.755-.165.165-.337.328-.517.486l.708.709z"/>
                                    <path d="M11.297 9.176a3.5 3.5 0 0 0-4.474-4.474l.823.823a2.5 2.5 0 0 1 2.829 2.829l.822.822zm-2.943 1.299.822.822a3.5 3.5 0 0 1-4.474-4.474l.823.823a2.5 2.5 0 0 0 2.829 2.829z"/>
                                    <path d="M3.35 5.47c-.18.16-.353.322-.518.487A13.134 13.134 0 0 0 1.172 8l.195.288c.335.48.83 1.12 1.465 1.755C4.121 11.332 5.881 12.5 8 12.5c.716 0 1.39-.133 2.02-.36l.77.772A7.029 7.029 0 0 1 8 13.5C3 13.5 0 8 0 8s.939-1.721 2.641-3.238l.708.709zm10.296 8.884-12-12 .708-.708 12 12-.708.708z"/>
                                </svg>
                            }.into_any()
                        }}
                    </button>
                </span>
                {move || {
                    if show_splits.get() {
                        view! {
                            {move || {
                                let pace = pace_get.get();
                                let distance = distance_read.get();
                                let splits = splits_get.get();
                                if pace > Duration::ZERO && distance > 0 && splits > 0 {
                                    let mut entries = Vec::new();
                                    for i in (splits..=distance as usize).step_by(splits) {
                                        let split_duration_secs = pace.as_secs_f64()
                                            * (i as f64 / 1000.0);
                                        let total_seconds = split_duration_secs as u32;
                                        let minutes = total_seconds / 60;
                                        let seconds = total_seconds % 60;
                                        entries
                                            .push(
                                                view! {
                                                    <div style="white-space: nowrap; display: inline-block;">
                                                        {format!("{}m: {:02}:{:02}", i, minutes, seconds)}
                                                    </div>
                                                },
                                            );
                                    }
                                    entries
                                } else {
                                    Vec::new()
                                }
                            }}
                        }
                            .into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (forms, set_forms) = signal(vec![0]);
    let (next_id, set_next_id) = signal(1);

    let add_form = move |_| {
        set_forms.update(|forms| {
            forms.push(next_id.get());
        });
        set_next_id.update(|id| *id += 1);
    };

    let delete_form = Callback::new(move |id: usize| {
        set_forms.update(|forms| {
            if let Some(pos) = forms.iter().position(|&form_id| form_id == id) {
                forms.remove(pos);
            }
        });
    });

    view! {
        <div>
            <h1>"Pace Calculator"</h1>
            <div>
                {move || {
                    forms
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, id)| {
                            let delete_option = if index > 0 {
                                Some(delete_form.clone())
                            } else {
                                None
                            };

                            view! { <PaceCalculatorForm id=id on_delete=delete_option /> }
                        })
                        .collect_view()
                }}
            </div>
            <div style="margin-top: 20px;">
                <button
                    on:click=add_form
                    style="background-color: #4CAF50; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 18px;"
                >
                    "+"
                </button>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(|| view! { <App /> });
}
