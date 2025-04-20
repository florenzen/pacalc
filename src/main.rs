use leptos::prelude::*;
use leptos::*;
use std::collections::HashMap;

mod colors;
mod components;
mod form_state;

use components::*;
use form_state::FormState;

const TITLE: &str = "Pace Calculator";

#[component]
fn App() -> impl IntoView {
    let _ = document().set_title(TITLE);

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

                {format!("body {{ color: {}; }}", colors::BLUE2)}
                {format!(
                    "input {{ border-radius: 3px; outline: none; padding: 3px; border: 0px; color: {}; border-color: {}; background-color: {}; }}",
                    colors::WHITE,
                    colors::GREY,
                    colors::BLUE4,
                )}
            </style>
            <div>
                <h1>{TITLE}</h1>
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
                        style=format!(
                            "background-color:{}; color: white; border: none; padding: 6px 12px; border-radius: 5px; cursor: pointer; font-size: 18px;",
                            colors::BLUE4,
                        )
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
