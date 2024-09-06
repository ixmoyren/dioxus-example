use dioxus::prelude::*;

#[component]
pub fn Button(on_click: Option<EventHandler<MouseEvent>>, children: Element) -> Element {
    rsx! {
        button {
            class: "button",
            onclick: move |event| {
                if let Some(on_click) = on_click {
                    on_click.call(event);
                }
            },
            {children}
        }
    }
}
