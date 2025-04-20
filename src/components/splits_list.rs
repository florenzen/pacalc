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
