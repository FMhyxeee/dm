use dioxus::prelude::*;

use crate::Route;

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
                Sidebar {
                    is_open:  true,
                }
                // div {
                //     class: "w-64 bg-white shadow-lg",
                //     // 侧边栏内容
                //     div {
                //         class: "p-4",
                //         h2 { class: "text-lg font-semibold", "侧边栏菜单" },
                //         ul {
                //             class: "mt-4",
                //             li { Link { to: Route::Home{}, class: "block py-2 hover:bg-gray-100", "首页" } },
                //             li { Link { to: Route::Settings{}, class: "block py-2 hover:bg-gray-100", "设置" } },
                //             // 添加更多菜单项...
                //         }
                //     }
                // }

                // 内容区域
                div {
                    class: "flex-1 overflow-auto p-4 transition-all duration-300",
                    id: "content",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct SidebarProps {
    is_open: bool,
}

#[component]
fn Sidebar(props: SidebarProps) -> Element {
    let mut is_open = use_signal(|| props.is_open);
    rsx! {
         // 左侧边栏
         div {
            class: "w-64 overflow-y-auto transition-all duration-300 transform bg-gray-900",
            class: if *is_open.read() { "translate-x-0" } else { "-translate-x-full" },
            // 侧边栏内容
            nav {
                class: "mt-10",
                a {
                    class: "flex items-center px-6 py-2 mt-4 text-gray-100 hover:bg-gray-700 hover:bg-opacity-25 hover:text-gray-100",
                    href: "#",
                    span { class: "mx-3", "Dashboard" }
                }
                a {
                    class: "flex items-center px-6 py-2 mt-4 text-gray-100 hover:bg-gray-700 hover:bg-opacity-25 hover:text-gray-100",
                    href: "#",
                    span { class: "mx-3", "用户列表" }
                }
                // 添加更多菜单项...
            }

            button {
                class: "absolute top-4 -right-3 bg-gray-900 text-gray-100 rounded-full p-1 focus:outline-none",
                onclick: move |_| {
                    let toggle = *is_open.read();
                    *is_open.write() = !toggle;
                },
                svg {
                    class: "w-4 h-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: if *is_open.read() { "M15 19l-7-7 7-7" } else { "M9 5l7 7-7 7" }
                    }
                }
            }
        }
    }
}
