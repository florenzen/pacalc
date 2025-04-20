use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn SplitsList(
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
