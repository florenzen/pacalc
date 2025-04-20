use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn TotalDuration(total_duration: Memo<Option<Duration>>) -> impl IntoView {
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
