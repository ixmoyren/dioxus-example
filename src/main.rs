mod button;
mod form;
mod friend;

use crate::button::Button;
use crate::form::{FormAddFriend, FormSplitBill};
use crate::friend::{Friend, FriendsList};
use bigdecimal::BigDecimal;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use uuid::Uuid;

const FAVICON: &str = asset!("assets/favicon.ico");
const STYLE: &str = asset!("assets/main.css");
const _AVATAR: &str = asset!("assets/avatar");

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Title { "Dioxus Example" }
        Meta { charset: "utf-8" }
        Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        Meta { name: "theme-color", content: "#000000" }
        Meta { name: "description", content: "Web site created using dioxus-cli" }
        head::Link { rel: "icon", href: FAVICON }
        head::Link { rel: "stylesheet", href: STYLE }
        EatNSplit {}
    }
}

#[component]
fn EatNSplit() -> Element {
    let mut friends = use_signal(Vec::<Friend>::new);
    let mut show_add_friend = use_signal(|| false);
    let mut select_friend = use_signal(|| None::<Friend>);
    use_effect(move || {
        friends.extend(vec![
            Friend::default()
                .name("Clark")
                .image("avatar/Clark.jpg")
                .balance(BigDecimal::from(-7)),
            Friend::default()
                .name("Sarah")
                .image("avatar/Sarah.jpg")
                .balance(BigDecimal::from(20)),
            Friend::default()
                .name("Anthony")
                .image("avatar/Anthony.jpg")
                .balance(BigDecimal::from(0)),
        ])
    });
    let handle_add_friend = move |friend: Friend| {
        friends.push(friend);
    };
    let handle_remove_friend = move |id: Uuid| {
        friends.retain(|friend| friend.id != id);
    };
    let handle_show_friend = move |_data: MouseEvent| {
        show_add_friend.set(!show_add_friend());
    };
    let handle_select_friend = move |friend: Option<Friend>| {
        select_friend.set(friend);
        show_add_friend.set(false);
    };
    let handle_split_bill = move |(id, paid_bill): (Uuid, BigDecimal)| {
        if let Some(mut friend) = friends.iter_mut().find(|p| p.id == id) {
            info!("{}", friend.balance);
            friend.balance += paid_bill;
        }
    };
    rsx! {
        div { class: "app",
            div { class: "sidebar",
                FriendsList {
                    friends: friends(),
                    select_friend: select_friend(),
                    handle_select_friend,
                    handle_remove_friend,
                }
                if show_add_friend() {
                    FormAddFriend { handle_add_friend }
                }
                Button { on_click: handle_show_friend,
                    if show_add_friend() {
                        "Close"
                    } else {
                        "Add friend"
                    }
                }
            }
            if let Some(friend) = select_friend() {
                FormSplitBill { friend, handle_split_bill, key: "{friend.id}" }
            }
        }
    }
}
