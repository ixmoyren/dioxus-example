use super::Button;
use super::Friend;
use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use uuid::Uuid;

#[component]
pub fn FormAddFriend(handle_add_friend: EventHandler<Friend>) -> Element {
    let mut name = use_signal(|| None::<String>);
    let mut image = use_signal(|| None::<String>);
    let add_a_friend = move |_event| {
        if let (Some(add_name), Some(add_image)) = (name(), image()) {
            let new_friend = Friend::default().name(add_name).image(add_image);
            handle_add_friend.call(new_friend);
            name.set(None);
            image.set(None);
        }
    };
    let handle_input_name = move |event: FormEvent| {
        let event_name = event.value();
        if !event_name.trim().is_empty() {
            name.set(Some(event_name));
        }
    };
    let handle_input_image = move |event: FormEvent| {
        let event_image = event.value();
        if !event_image.trim().is_empty() {
            image.set(Some(event_image));
        }
    };

    rsx! {
        form { class: "form-add-friend", onsubmit: add_a_friend,
            label { "🧑‍🤝‍🧑 Friend name" }
            input { r#type: "text", value: name, onchange: handle_input_name }
            label { "🏜️ Image URL" }
            input { r#type: "text", value: image, onchange: handle_input_image }
            Button { "Add" }
        }
    }
}

#[component]
pub fn FormSplitBill(
    friend: Friend,
    handle_split_bill: EventHandler<(Uuid, BigDecimal)>,
) -> Element {
    let mut bill = use_signal(|| None::<BigDecimal>);
    let mut paid_by_user = use_signal(BigDecimal::zero);
    let mut who_is_paying = use_signal(|| "user".to_owned());
    let Friend {
        id,
        name,
        image: _,
        balance: _,
    } = friend;
    let paid_by_friend = bill().as_ref().map(|bill| bill - &paid_by_user());
    let paid_by_friend_ = paid_by_friend.clone();
    let handle_update_bill = move |event: FormEvent| {
        let event_bill = event.parsed::<BigDecimal>().expect("Acquire bill error!");
        bill.set(Some(event_bill));
    };
    let handle_update_paid_by_user = move |event: FormEvent| {
        let event_paid_by = event
            .parsed::<BigDecimal>()
            .expect("Acquire your expanse error!");
        let old_paid_by_user = paid_by_user();
        match &bill() {
            Some(ref bill) if &event_paid_by <= bill => paid_by_user.set(event_paid_by),
            _ => paid_by_user.set(old_paid_by_user),
        }
    };
    let handle_update_who_is_paying = move |event: FormEvent| {
        let event_who_is_paying = event.value();
        who_is_paying.set(event_who_is_paying);
    };
    let onsubmit = move |event: FormEvent| {
        let web_event = event.as_web_event();
        web_event.prevent_default();
        if bill().is_none() {
            return;
        }
        let paid_bill = if "user" == who_is_paying() {
            if let Some(paid_by) = &paid_by_friend_ {
                paid_by.clone()
            } else {
                BigDecimal::zero()
            }
        } else {
            -paid_by_user()
        };
        handle_split_bill.call((id, paid_bill));
    };
    let handle_clear_from = move |event: MouseEvent| {
        // 停止事件传播
        event.stop_propagation();
        // 清理表单信息
        bill.set(None);
        paid_by_user.set(BigDecimal::zero());
        who_is_paying.set("user".to_owned());
    };
    rsx! {
        form { class: "form-split-bill", onsubmit,
            h2 { "Split a bill with {name}" }
            label { "💵 Bill value" }
            input {
                r#type: "text",
                value: if let Some(bill) = bill() { "{bill}" } else { "".to_owned() },
                onchange: handle_update_bill,
            }
            label { "🧍‍♂️ Your expense" }
            input {
                r#type: "text",
                value: "{paid_by_user}",
                onchange: handle_update_paid_by_user,
            }
            label { "🧑‍🤝‍🧑 {name}'s expense" }
            input {
                r#type: "text",
                disabled: true,
                value: if let Some(paid_by_friend) = paid_by_friend { "{paid_by_friend}" } else { "".to_owned() },
            }
            label { "🤑 Who is paying the bill" }
            select { value: who_is_paying, onchange: handle_update_who_is_paying,
                option { value: "user", "You" }
                option { value: "friend", {name} }
            }
            Button { "Split bill" }
            Button { on_click: handle_clear_from, "Clear" }
        }
    }
}
