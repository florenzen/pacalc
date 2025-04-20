use crate::colors;
use leptos::prelude::*;

#[component]
pub fn ErrorMessage(error_message_get: ReadSignal<String>) -> impl IntoView {
    view! {
        <div
            style:color="red"
            style:background-color=colors::GREY
            style:border-radius="5px"
            style:padding=move || if error_message_get.get().is_empty() { "0" } else { "5px" }
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
