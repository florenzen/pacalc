use leptos::prelude::*;
use leptos::*;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
struct FormState {
    pace: Duration,
    splits: usize,
    distance: usize,
    show_splits: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            pace: Duration::ZERO,
            splits: 0,
            distance: 0,
            show_splits: true,
        }
    }
}

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
fn PaceInput(
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

#[component]
fn SplitsInput(
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

#[component]
fn DistanceInput(
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
                                    .set(
                                        "Distance must be a positive number".to_string(),
                                    );
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

#[component]
fn TotalDuration(total_duration: Memo<Option<Duration>>) -> impl IntoView {
    view! {
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
                    .unwrap_or_else(|| "—".to_string())
            }}
        </div>
    }
}

#[component]
fn ErrorMessage(error_message_get: ReadSignal<String>) -> impl IntoView {
    view! {
        <div
            style:color="red"
            style:height=move || if error_message_get.get().is_empty() { "0" } else { "auto" }
            style:overflow="hidden"
            style:margin-bottom=move || {
                if error_message_get.get().is_empty() { "0" } else { "10px" }
            }
        >
            {move || error_message_get.get()}
        </div>
    }
}

#[component]
fn SplitToggle(
    id: usize,
    show_splits_get: ReadSignal<bool>,
    show_splits_set: WriteSignal<bool>,
    set_form_states: WriteSignal<HashMap<usize, FormState>>,
) -> impl IntoView {
    view! {
        <span style="display: inline-flex; align-items: center; gap: 5px;">
            "Splits:"
            <button
                on:click=move |_| {
                    show_splits_set.set(!show_splits_get.get());
                    set_form_states
                        .update(|states| {
                            if let Some(state) = states.get_mut(&id) {
                                state.show_splits = show_splits_get.get();
                            }
                        });
                }
                style="height: 26px; background: transparent; border: none; padding: 5px; border-radius: 3px; cursor: pointer; color: #777;"
            >
                {move || {
                    if show_splits_get.get() {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                fill="currentColor"
                                viewBox="0 0 16 16"
                            >
                                <path d="M16 8s-3-5.5-8-5.5S0 8 0 8s3 5.5 8 5.5S16 8 16 8zM1.173 8a13.133 13.133 0 0 1 1.66-2.043C4.12 4.668 5.88 3.5 8 3.5c2.12 0 3.879 1.168 5.168 2.457A13.133 13.133 0 0 1 14.828 8c-.058.087-.122.183-.195.288-.335.48-.83 1.12-1.465 1.755C11.879 11.332 10.119 12.5 8 12.5c-2.12 0-3.879-1.168-5.168-2.457A13.134 13.134 0 0 1 1.172 8z" />
                                <path d="M8 5.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5zM4.5 8a3.5 3.5 0 1 1 7 0 3.5 3.5 0 0 1-7 0z" />
                            </svg>
                        }
                            .into_any()
                    } else {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                fill="currentColor"
                                viewBox="0 0 16 16"
                            >
                                <path d="M13.359 11.238C15.06 9.72 16 8 16 8s-3-5.5-8-5.5a7.028 7.028 0 0 0-2.79.588l.77.771A5.944 5.944 0 0 1 8 3.5c2.12 0 3.879 1.168 5.168 2.457A13.134 13.134 0 0 1 14.828 8c-.058.087-.122.183-.195.288-.335.48-.83 1.12-1.465 1.755-.165.165-.337.328-.517.486l.708.709z" />
                                <path d="M11.297 9.176a3.5 3.5 0 0 0-4.474-4.474l.823.823a2.5 2.5 0 0 1 2.829 2.829l.822.822zm-2.943 1.299.822.822a3.5 3.5 0 0 1-4.474-4.474l.823.823a2.5 2.5 0 0 0 2.829 2.829z" />
                                <path d="M3.35 5.47c-.18.16-.353.322-.518.487A13.134 13.134 0 0 0 1.172 8l.195.288c.335.48.83 1.12 1.465 1.755C4.121 11.332 5.881 12.5 8 12.5c.716 0 1.39-.133 2.02-.36l.77.772A7.029 7.029 0 0 1 8 13.5C3 13.5 0 8 0 8s.939-1.721 2.641-3.238l.708.709zm10.296 8.884-12-12 .708-.708 12 12-.708.708z" />
                            </svg>
                        }
                            .into_any()
                    }
                }}
            </button>
        </span>
    }
}

#[component]
fn SplitsList(
    pace_get: ReadSignal<Duration>,
    distance_get: ReadSignal<usize>,
    splits_get: ReadSignal<usize>,
    show_splits_get: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 10px; margin-left: 10px; justify-content: flex-start; align-items: baseline;">
            {move || {
                if show_splits_get.get() {
                    {
                        move || {
                            let pace = pace_get.get();
                            let distance = distance_get.get();
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
                        }
                    }
                        .into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn DeleteButton(id: usize, callback: Callback<usize>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| callback.run(id)
            style="background: transparent; border: none; padding: 5px; border-radius: 3px; cursor: pointer; color: #ff4d4d;"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                fill="currentColor"
                viewBox="0 0 16 16"
            >
                <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
            </svg>
        </button>
    }
}

#[component]
fn PaceCalculatorForm(
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
            class="calculator-form"
            style="border: 1px solid #ccc; padding: 15px; margin-bottom: 20px; border-radius: 5px;"
        >
            <div style="display: flex; flex-wrap: wrap; gap: 20px; margin-bottom: 15px; align-items: flex-start; justify-content: space-between;">
                <div style="display: flex; flex: 1; flex-wrap: wrap; gap: 20px; align-items: center;">
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
                <div style="margin-left: auto; align-self: flex-start;">
                    {move || match on_delete.clone() {
                        Some(callback) => {
                            view! { <DeleteButton id=id callback=callback /> }.into_any()
                        }
                        None => view! { <></> }.into_any(),
                    }}
                </div>
            </div>
            <ErrorMessage error_message_get=error_message_get />
            <div style="display: flex; align-items: baseline;">
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

#[component]
fn App() -> impl IntoView {
    let (forms_get, forms_set) = signal(vec![0]);
    let (next_id_get, next_id_set) = signal(1);

    let (form_states_get, form_states_set) = signal(HashMap::<usize, FormState>::new());

    form_states_set.update(|states| {
        states.insert(0, FormState::default());
    });

    let add_form = move |_| {
        let new_id = next_id_get.get();
        forms_set.update(|forms| {
            forms.push(new_id);
        });
        form_states_set.update(|states| {
            states.insert(new_id, FormState::default());
        });
        next_id_set.update(|id| *id += 1);
    };

    let delete_form = Callback::new(move |id: usize| {
        forms_set.update(|forms| {
            if let Some(pos) = forms.iter().position(|&form_id| form_id == id) {
                forms.remove(pos);
            }
        });
        form_states_set.update(|states| {
            states.remove(&id);
        });
    });

    view! {
        <>
            <style>
                "@import url('https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;600;700&display=swap');"
                
                "body, button, input, div, span { font-family: 'Open Sans', system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Oxygen, Ubuntu, Cantarell, sans-serif; }"
            </style>
            <div>
                <h1>"Pace Calculator"</h1>
                <div>
                    {move || {
                        forms_get
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(index, id)| {
                                let delete_option = if index > 0 {
                                    Some(delete_form.clone())
                                } else {
                                    None
                                };
                                let form_state = Memo::new(move |_| {
                                    form_states_get
                                        .with(|states| states.get(&id).cloned().unwrap_or_default())
                                });

                                view! {
                                    <PaceCalculatorForm
                                        id=id
                                        on_delete=delete_option
                                        form_state=form_state
                                        set_form_states=form_states_set.clone()
                                    />
                                }
                            })
                            .collect_view()
                    }}
                </div>
                <div style="margin-top: 15px;">
                    <button
                        on:click=add_form
                        style="background-color: #4CAF50; color: white; border: none; padding: 6px 12px; border-radius: 5px; cursor: pointer; font-size: 18px;"
                    >
                        "+"
                    </button>
                </div>
            </div>
        </>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(|| view! { <App /> });
}
