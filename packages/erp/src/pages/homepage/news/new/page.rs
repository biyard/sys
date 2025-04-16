use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn NewsNewPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let _tr: NewsNewTranslate = translate(&lang);
    let error = use_signal(String::new);
    let success = use_signal(|| false);

    rsx! {
        form {
            onsubmit: move |_| async move {
                ctrl.submit().await;
            },

            h2 { class: "text-xl font-bold", "Create News" }

            InputField { label: "Category", state: ctrl.category }
            InputField { label: "Title", state: ctrl.title }
            InputField { label: "Image URL", state: ctrl.image }
            InputTextarea { label: "Contents (max 350)", state: ctrl.contents }
            InputField { label: "Link", state: ctrl.link }

            label { class: "row",
                input {
                    class: "w-fit",
                    r#type: "checkbox",
                    checked: ctrl.main(),
                    onchange: move |evt| ctrl.main.set(evt.value().parse().unwrap_or(false)),
                }
                span { class: "w-full", "Set as main" }
            }

            if !error().is_empty() {
                p { class: "text-red-500 text-sm", "{error()}" }
            }

            if *success.read() {
                p { class: "text-green-500 text-sm", "News created successfully!" }
            }

            button { r#type: "submit", "Create" }
        }
    }
}

#[component]
fn InputField(label: String, state: Signal<String>) -> Element {
    rsx! {
        div {
            label { "{label}" }
            input {
                r#type: "text",
                value: state(),
                oninput: move |e| state.set(e.value()),
            }
        }
    }
}

#[component]
fn InputTextarea(label: String, state: Signal<String>) -> Element {
    rsx! {
        div {
            label { "{label}" }
            textarea {
                value: state(),
                rows: 5,
                oninput: move |e| state.set(e.value()),
            }
        }
    }
}
