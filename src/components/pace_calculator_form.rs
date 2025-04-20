use crate::colors;
use crate::components::*;
use crate::form_state::FormState;
use leptos::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

#[component]
pub fn PaceCalculatorForm(
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
            style=format!(
                "border: 1px solid #ccc; padding: 15px; margin-bottom: 20px; border-radius: 5px; color: {}; background-color: {}",
                colors::WHITE,
                colors::BLUE1,
            )
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
