use leptos::logging::log;
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
fn App() -> impl IntoView {
    let (pace_get, pace_set) = signal(Duration::ZERO);
    let (splits_get, splits_set) = signal(0);
    let (distance_read, distance_write) = signal(0);
    let total_duration = Memo::new(move |_| {
        let pace = pace_get.get();
        let distance = distance_read.get();
        logging::log!("Pace: {:?}, splits: {}", pace, distance);
        let seconds =
            if pace > Duration::ZERO && distance > 0 {
                Some((distance as f64 / 1000.0) * pace.as_secs_f64())
            } else {
                None
            };
        seconds.map(Duration::from_secs_f64)
    });

    view! {
        <div>
            <h1>"Lap Time Calculator"</h1>
            <label>
                "Pace (mm:ss/km): "
                <input
                    type="text"
                    on:input=move |ev| {
                        let pace_str = event_target_value(&ev);
                        match parse_duration(&pace_str) {
                            Ok(duration) => pace_set.set(duration),
                            Err(msg) => {
                                logging::warn!(
                                    "Failed to parse pace duration {}: {}", pace_str, msg
                                )
                            }
                        }
                    }
                />
            </label>
            <label>
                "Splits (m): "
                <input
                    type="number"
                    on:input=move |ev| {
                        splits_set.set(event_target_value(&ev).parse().unwrap_or(0))
                    }
                />
            </label>
            <label>
                "Distance (m): "
                <input
                    type="number"
                    on:input=move |ev| {
                        distance_write.set(event_target_value(&ev).parse().unwrap_or(0))
                    }
                />
            </label>
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
            <table>
                <thead>
                    <tr>
                        <th>"Distance (m)"</th>
                        <th>"Time (mm:ss)"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let pace = pace_get.get();
                        let distance = distance_read.get();
                        let splits = splits_get.get();
                        if pace > Duration::ZERO && distance > 0 && splits > 0 {
                            let mut rows = Vec::new();
                            for i in (splits..=distance as usize).step_by(splits) {
                                let split_duration_secs = pace.as_secs_f64() * (i as f64 / 1000.0);
                                let total_seconds = split_duration_secs as u32;
                                let minutes = total_seconds / 60;
                                let seconds = total_seconds % 60;
                                rows.push(view! {
                                    <tr>
                                        <td>{i}</td>
                                        <td>{format!("{:02}:{:02}", minutes, seconds)}</td>
                                    </tr>
                                });
                            }
                            rows
                        } else {
                            Vec::new()
                        }
                    }}
                </tbody>
            </table>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(|| view! { <App /> });
}
