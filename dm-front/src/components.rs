use dioxus::prelude::*;
use reqwest::Client;

use crate::Route;
use crate::model::UserDetail;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-screen",

            // 顶部导航栏
            nav {
                class: "bg-gray-800 p-4 shadow-md",
                div {
                    class: "container mx-auto flex justify-between items-center",
                    Link {
                        to: Route::Home{},
                        class: "text-white hover:text-gray-300 px-3 py-2 rounded-md text-sm font-medium",
                        "首页"
                    },
                    Link {
                        to: Route::Settings{},
                        class: "text-white hover:text-gray-300 px-3 py-2 rounded-md text-sm font-medium",
                        "设置"
                    }
                }
            }

            // 主要内容区域
            div {
                class: "flex flex-1 overflow-hidden",

                // 左侧边栏
                div {
                    class: "w-64 bg-white shadow-lg",
                    // 侧边栏内容
                    div {
                        class: "p-4",
                        h2 { class: "text-lg font-semibold", "侧边栏菜单" },
                        ul {
                            class: "mt-4",
                            li { Link { to: Route::Home{}, class: "block py-2 hover:bg-gray-100", "首页" } },
                            li { Link { to: Route::Settings{}, class: "block py-2 hover:bg-gray-100", "设置" } },
                            // 添加更多菜单项...
                        }
                    }
                }

                // 内容区域
                div {
                    class: "flex-1 overflow-auto p-4",
                    id: "content",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
pub fn Settings() -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Settings"
    }
}

#[component]
pub fn Home() -> Element {
    let future: Resource<Vec<UserDetail>> = use_resource(|| async move {
        let client = Client::new();
        let res = client
            .get("http://127.0.0.1:3000/list_all")
            .send()
            .await
            .unwrap();
        let list = res.json().await.unwrap();

        list
    });

    match &*future.read_unchecked() {
        Some(users) => {
            rsx! {
                h1 { class: "text-2xl font-bold mb-4", "用户列表" }
                table {
                    class: "min-w-full bg-white border border-gray-300",
                    thead {
                        tr {
                            class: "bg-gray-100",
                            th { class: "py-2 px-4 border-b", "ID" }
                            th { class: "py-2 px-4 border-b", "AGE" }
                            th { class: "py-2 px-4 border-b", "SALARY" }
                        }
                    }
                    tbody {
                        {
                            users.iter().map(|user| {
                                rsx! {
                                    tr {
                                        key: "{user.id}",
                                        td { class: "py-2 px-4 border-b", "{user.id}" }
                                        td { class: "py-2 px-4 border-b", "{user.age}" }
                                        td { class: "py-2 px-4 border-b", "{user.salary}" }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "text-center py-4", "正在加载..." }
            }
        }
    }
}
