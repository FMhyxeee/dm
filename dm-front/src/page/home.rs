use std::collections::HashMap;

use dioxus::prelude::*;
use reqwest::Client;

use crate::model::UserDetail;

#[component]
pub fn Home() -> Element {
    let mut users = use_signal(|| Vec::<UserDetail>::new());
    let mut editing = use_signal(|| HashMap::<(usize, String), bool>::new());
    let mut temp_value = use_signal(String::new);

    let future: Resource<bool> = use_resource(move || async move {
        let client = Client::new();
        let res = client
            .get("http://127.0.0.1:3000/list_all")
            .send()
            .await
            .unwrap();
        let list = res.json().await.unwrap();

        users.set(list);
        true
    });

    match &*future.read_unchecked() {
        Some(true) => {
            rsx! {
                h1 { class: "text-2xl font-bold mb-4", "用户列表" }
                table {
                    class: "min-w-full bg-white border border-gray-300",
                    thead {
                        tr {
                            class: "bg-gray-100",
                            th { class: "border-gray-300 px-4 py-2 text-left w-1/12 border-b", "ID" }
                            th { class: "border-gray-300 px-4 py-2 text-left w-3/12 border-b", "NAME" }
                            th { class: "border-gray-300 px-4 py-2 text-right w-3/12 border-b", "AGE" }
                            th { class: "border-gray-300 px-4 py-2 text-right w-3/12 border-b", "SALARY" }
                        }
                    }
                    tbody {
                        {
                            users.iter().enumerate().map(|(i, user)| {
                                rsx! {
                                    tr {
                                        key: "{user.id}",
                                    }
                                    td { class: "border-gray-300 px-4 py-2 text-left w-1/12 border-b", "{user.id}"}
                                    td { class: "border-gray-300 px-4 py-2 text-left w-3/12 border-b","{user.name}"}
                                    td { class: "border-gray-300 px-4 py-2 text-right w-3/12 border-b",
                                        ondoubleclick: move |_| {
                                            editing.write().insert((i, "age".to_string()), true);
                                        },
                                        {
                                            if *editing.read().get(&(i, "age".to_string())).unwrap_or(&false) {
                                                rsx! {
                                                    input {
                                                        value: "{user.age}",
                                                        oninput:  move |e| temp_value.set(e.value().clone()),
                                                        onblur: move |_| {
                                                             if *editing.write().get(&(i, "age".to_string())).unwrap_or(&false) {
                                                                users.write()[i].age = temp_value.read().parse::<i32>().unwrap();
                                                             }
                                                             editing.write().remove(&(i, "age".to_string())).unwrap_or(false);

                                                        }
                                                    }
                                                }
                                            } else {
                                                rsx! {
                                                    "{user.age}"
                                                }
                                            }
                                        }}
                                    td { class: "border-gray-300 px-4 py-2 text-right w-3/12 border-b", "{user.salary}" }

                                }
                            })
                        }
                    }
                }
            }
        }
        _ => {
            rsx! {
                div { class: "text-center py-4", "正在加载..." }
            }
        }
    }
}
