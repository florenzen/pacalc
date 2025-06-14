// Copyright (c) 2025 Florian Lorenzen

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
