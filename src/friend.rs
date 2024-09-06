use super::Button;
use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use dioxus::prelude::*;
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Friend {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub balance: BigDecimal,
}

impl Default for Friend {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: "".to_owned(),
            image: "".to_owned(),
            balance: BigDecimal::zero(),
        }
    }
}

impl Friend {
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn image(mut self, url: impl ToString) -> Self {
        self.image = url.to_string();
        self
    }

    pub fn balance(mut self, balance: BigDecimal) -> Self {
        self.balance = balance;
        self
    }
}

#[component]
pub fn FriendsList(
    friends: Vec<Friend>,
    select_friend: Option<Friend>,
    handle_select_friend: EventHandler<Option<Friend>>,
    handle_remove_friend: EventHandler<Uuid>,
) -> Element {
    let friend_li = friends.iter().map(|friend| {
        let friend = friend.clone();
        rsx! {
            FriendLi {
                friend,
                select_friend: select_friend.clone(),
                handle_select_friend,
                handle_remove_friend,
            }
        }
    });
    rsx! {
        ul { {friend_li} }
    }
}

#[component]
fn FriendLi(
    friend: Friend,
    select_friend: Option<Friend>,
    handle_select_friend: EventHandler<Option<Friend>>,
    handle_remove_friend: EventHandler<Uuid>,
) -> Element {
    let Friend {
        id,
        name,
        image,
        balance,
    } = friend.clone();
    let (li_class, select_button_msg) = match select_friend {
        Some(friend) if friend.id == id => ("selected", "Close"),
        _ => ("", "Select"),
    };
    let on_select_friend = move |_event: MouseEvent| {
        let friend = if select_button_msg != "Close" {
            Some(friend.clone())
        } else {
            None
        };
        handle_select_friend.call(friend);
    };
    let on_remove_friend = move |_event: MouseEvent| {
        handle_remove_friend.call(id);
        handle_select_friend.call(None);
    };
    let (colour, msg) = match &balance.cmp(&0.into()) {
        Ordering::Less => (
            "red".to_owned(),
            format!("You owe {name} ${}", &balance.abs()),
        ),
        Ordering::Equal => ("green".to_owned(), format!("You and {name} are even")),
        Ordering::Greater => ("".to_owned(), format!("{name} owe you ${}", &balance.abs())),
    };
    rsx! {
        li { id: "li-{id}", class: li_class,
            img { src: image, alt: name.clone() }
            h3 { {name.clone()} }
            p { class: colour, {msg} }
            Button { on_click: on_select_friend, {select_button_msg} }
            Button { on_click: on_remove_friend, "Remove" }
        }
    }
}
