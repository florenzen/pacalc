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
fn PaceCalculatorForm(
    id: usize,
    on_delete: Option<Callback<usize>>,
) -> impl IntoView {
    let (pace_get, pace_set) = signal(Duration::ZERO);
    let (splits_get, splits_set) = signal(0);
    let (distance_read, distance_write) = signal(0);
    let (show_splits, set_show_splits) = signal(true);
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
        <div class="calculator-form" style="border: 1px solid #ccc; padding: 15px; margin-bottom: 20px; border-radius: 5px;">
            <div style="display: flex; flex-wrap: wrap; gap: 20px; margin-bottom: 15px; align-items: flex-start; justify-content: space-between;">
                <div style="display: flex; flex: 1; flex-wrap: wrap; gap: 20px; align-items: center;">
                    <div>
                        <label>
                            "Pace (mm:ss/km): "
                            <input
                                type="text"
                                on:input=move |ev| {
                                    let pace_str = event_target_value(&ev);
                                    match parse_duration(&pace_str) {
                                        Ok(duration) => pace_set.set(duration),
                                        Err(_) => {}
                                    }
                                }
                            />
                        </label>
                    </div>
                    <div>
                        <label>
                            "Splits (m): "
                            <input
                                type="number"
                                on:input=move |ev| {
                                    splits_set.set(event_target_value(&ev).parse().unwrap_or(0))
                                }
                            />
                        </label>
                    </div>
                    <div>
                        <label>
                            "Distance (m): "
                            <input
                                type="number"
                                on:input=move |ev| {
                                    distance_write.set(event_target_value(&ev).parse().unwrap_or(0))
                                }
                            />
                        </label>
                    </div>
                </div>
                <div style="margin-left: auto; align-self: flex-start;">
                    {move || match on_delete.clone() {
                        Some(callback) => view! {
                            <button 
                                on:click=move |_| callback.run(id)
                                class="delete-btn"
                                style="background-color: #ff4d4d; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer;"
                            >
                                "Delete"
                            </button>
                        }.into_any(),
                        None => view! { <></> }.into_any(),
                    }}
                </div>
            </div>
            <p>
                "Total duration: "
                {move || {
                    total_duration
                        .get()
                        .map(|time| {
                            let total_seconds = time.as_secs();
                            let minutes = total_seconds / 60;
                            let seconds = total_seconds % 60;
                            format!("{:02}:{:02} minutes", minutes, seconds)
                        })
                        .unwrap_or_else(|| "N/A".to_string())
                }}
            </p>
            <div style="display: flex; flex-wrap: wrap; align-items: center; gap: 20px; margin-top: 15px;">
                <button on:click=move |_| set_show_splits.set(!show_splits.get())>
                    {move || if show_splits.get() { "Hide Splits" } else { "Show Splits" }}
                </button>
                {move || if show_splits.get() {
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
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
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
                    forms.get().into_iter().enumerate().map(|(index, id)| {
                        let delete_option = if index > 0 {
                            Some(delete_form.clone())
                        } else {
                            None
                        };
                        
                        view! {
                            <PaceCalculatorForm 
                                id=id 
                                on_delete=delete_option
                            />
                        }
                    }).collect_view()
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
