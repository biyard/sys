use dioxus::prelude::*;
#[component]
pub fn AlignFromLeft(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    width: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M4 5V19M8 12L20 12M20 12L17 9M20 12L17 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignFromRight(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M20 5V19M16 12H4M4 12L7 15M4 12L7 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignHorizontalCenter(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "none".to_string())]
    fill: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M12 10V14M12 4V5M12 19V20M9 10H15C15.5523 10 16 9.55228 16 9V6C16 5.44772 15.5523 5 15 5H9C8.44772 5 8 5.44772 8 6V9C8 9.55228 8.44772 10 9 10ZM7 19H17C17.5523 19 18 18.5523 18 18V15C18 14.4477 17.5523 14 17 14H7C6.44772 14 6 14.4477 6 15V18C6 18.5523 6.44772 19 7 19Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignToCenter(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M19 12H5M12 20L12 15M12 15L14 17M12 15L10 17M12 4V9M12 9L10 7M12 9L14 7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignLeft(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "".to_string())]
    class: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M5 6H19M5 10H15M5 14H19M5 18H15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignJustify(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M5 6H19M5 10H19M5 14H19M5 18H19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignToLeft(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M4 5V19M20 12H8M8 12L11 15M8 12L11 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignToTop(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "".to_string())]
    class: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M19 4H5M12 20L12 8M12 8L9 11M12 8L15 11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Merge(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M4 19L4 5M20 19L20 5M19 12H14M14 12L16 10M14 12L16 14M5 12H10M10 12L8 14M10 12L8 10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignVerticalTop(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M4 3C3.44772 3 3 3.44772 3 4C3 4.55228 3.44772 5 4 5V3ZM20 5C20.5523 5 21 4.55228 21 4C21 3.44772 20.5523 3 20 3V5ZM5.53903 15.8874L5.07805 16.7748L5.53903 15.8874ZM5.11259 15.461L4.22517 15.9219L5.11259 15.461ZM9.88741 15.461L10.7748 15.9219L9.88741 15.461ZM9.46097 15.8874L9.92195 16.7748L9.46097 15.8874ZM9.46097 8.11259L9.92195 7.22517L9.46097 8.11259ZM9.88741 8.53903L10.7748 8.07805L9.88741 8.53903ZM5.53903 8.11259L5.07805 7.22517L5.53903 8.11259ZM5.11259 8.53903L4.22517 8.07805L5.11259 8.53903ZM14.539 19.8874L14.0781 20.7748L14.539 19.8874ZM14.1126 19.461L13.2252 19.9219L14.1126 19.461ZM18.8874 19.461L19.7748 19.9219L18.8874 19.461ZM18.461 19.8874L18.9219 20.7748L18.461 19.8874ZM18.461 8.11259L18.9219 7.22517L18.461 8.11259ZM18.8874 8.53903L19.7748 8.07805L18.8874 8.53903ZM14.539 8.11259L14.0781 7.22517L14.539 8.11259ZM14.1126 8.53903L13.2252 8.07805L14.1126 8.53903ZM4 5H20V3H4V5ZM6.61 9H8.39V7H6.61V9ZM9 9.61V14.39H11V9.61H9ZM8.39 15H6.61V17H8.39V15ZM6 14.39V9.61H4V14.39H6ZM6.61 15C6.30847 15 6.14347 14.9992 6.02499 14.9894C5.91848 14.9805 5.94184 14.9698 6 15L5.07805 16.7748C5.35295 16.9176 5.62705 16.9632 5.85947 16.9825C6.07992 17.0008 6.34204 17 6.61 17V15ZM4 14.39C4 14.658 3.9992 14.9201 4.0175 15.1405C4.03681 15.373 4.08238 15.6471 4.22517 15.9219L6 15C6.03021 15.0582 6.01949 15.0815 6.01064 14.975C6.0008 14.8565 6 14.6915 6 14.39H4ZM6 15L4.22517 15.9219C4.41493 16.2872 4.71277 16.5851 5.07805 16.7748L6 15ZM9 14.39C9 14.6915 8.9992 14.8565 8.98936 14.975C8.98051 15.0815 8.96979 15.0582 9 15L10.7748 15.9219C10.9176 15.6471 10.9632 15.373 10.9825 15.1405C11.0008 14.9201 11 14.658 11 14.39H9ZM8.39 17C8.65796 17 8.92008 17.0008 9.14053 16.9825C9.37295 16.9632 9.64705 16.9176 9.92195 16.7748L9 15C9.05816 14.9698 9.08152 14.9805 8.97501 14.9894C8.85653 14.9992 8.69153 15 8.39 15V17ZM9 15L9.92195 16.7748C10.2872 16.5851 10.5851 16.2872 10.7748 15.9219L9 15ZM8.39 9C8.69153 9 8.85653 9.0008 8.97501 9.01064C9.08152 9.01949 9.05816 9.03021 9 9L9.92195 7.22517C9.64705 7.08238 9.37295 7.03681 9.14053 7.0175C8.92008 6.9992 8.65796 7 8.39 7V9ZM11 9.61C11 9.34204 11.0008 9.07992 10.9825 8.85947C10.9632 8.62705 10.9176 8.35295 10.7748 8.07805L9 9C8.96979 8.94184 8.98051 8.91848 8.98936 9.02499C8.9992 9.14347 9 9.30847 9 9.61H11ZM9 9L10.7748 8.07805C10.5851 7.71276 10.2872 7.41492 9.92195 7.22517L9 9ZM6.61 7C6.34204 7 6.07992 6.9992 5.85947 7.0175C5.62705 7.03681 5.35295 7.08238 5.07805 7.22517L6 9C5.94184 9.03021 5.91848 9.01949 6.02499 9.01064C6.14347 9.0008 6.30847 9 6.61 9V7ZM6 9.61C6 9.30847 6.0008 9.14347 6.01064 9.02499C6.01949 8.91848 6.03021 8.94184 6 9L4.22517 8.07805C4.08238 8.35295 4.03681 8.62705 4.0175 8.85947C3.9992 9.07992 4 9.34204 4 9.61H6ZM5.07805 7.22517C4.71276 7.41492 4.41492 7.71277 4.22517 8.07805L6 9L5.07805 7.22517ZM15.61 9H17.39V7H15.61V9ZM18 9.61V18.39H20V9.61H18ZM17.39 19H15.61V21H17.39V19ZM15 18.39V9.61H13V18.39H15ZM15.61 19C15.3085 19 15.1435 18.9992 15.025 18.9894C14.9185 18.9805 14.9418 18.9698 15 19L14.0781 20.7748C14.3529 20.9176 14.627 20.9632 14.8595 20.9825C15.0799 21.0008 15.342 21 15.61 21V19ZM13 18.39C13 18.658 12.9992 18.9201 13.0175 19.1405C13.0368 19.373 13.0824 19.6471 13.2252 19.9219L15 19C15.0302 19.0582 15.0195 19.0815 15.0106 18.975C15.0008 18.8565 15 18.6915 15 18.39H13ZM15 19L13.2252 19.9219C13.4149 20.2872 13.7128 20.5851 14.0781 20.7748L15 19ZM18 18.39C18 18.6915 17.9992 18.8565 17.9894 18.975C17.9805 19.0815 17.9698 19.0582 18 19L19.7748 19.9219C19.9176 19.6471 19.9632 19.373 19.9825 19.1405C20.0008 18.9201 20 18.658 20 18.39H18ZM17.39 21C17.658 21 17.9201 21.0008 18.1405 20.9825C18.373 20.9632 18.6471 20.9176 18.9219 20.7748L18 19C18.0582 18.9698 18.0815 18.9805 17.975 18.9894C17.8565 18.9992 17.6915 19 17.39 19V21ZM18 19L18.9219 20.7748C19.2872 20.5851 19.5851 20.2872 19.7748 19.9219L18 19ZM17.39 9C17.6915 9 17.8565 9.0008 17.975 9.01064C18.0815 9.01949 18.0582 9.03021 18 9L18.9219 7.22517C18.6471 7.08238 18.373 7.03681 18.1405 7.0175C17.9201 6.9992 17.658 7 17.39 7V9ZM20 9.61C20 9.34204 20.0008 9.07992 19.9825 8.85947C19.9632 8.62705 19.9176 8.35295 19.7748 8.07805L18 9C17.9698 8.94184 17.9805 8.91848 17.9894 9.02499C17.9992 9.14347 18 9.30847 18 9.61H20ZM18 9L19.7748 8.07805C19.5851 7.71277 19.2872 7.41493 18.9219 7.22517L18 9ZM15.61 7C15.342 7 15.0799 6.9992 14.8595 7.0175C14.627 7.03681 14.3529 7.08238 14.0781 7.22517L15 9C14.9418 9.03021 14.9185 9.01949 15.025 9.01064C15.1435 9.0008 15.3085 9 15.61 9V7ZM15 9.61C15 9.30847 15.0008 9.14347 15.0106 9.02499C15.0195 8.91848 15.0302 8.94184 15 9L13.2252 8.07805C13.0824 8.35295 13.0368 8.62705 13.0175 8.85947C12.9992 9.07992 13 9.34204 13 9.61H15ZM14.0781 7.22517C13.7128 7.41493 13.4149 7.71277 13.2252 8.07805L15 9L14.0781 7.22517Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn AlignToBottom(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "24".to_string())]
    width: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M5 20H19M12 4V16M12 16L15 13M12 16L9 13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VerticalDistribute(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "".to_string())]
    class: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M4 5L20 5M20 19H4M8 14H16C16.5523 14 17 13.5523 17 13V11C17 10.4477 16.5523 10 16 10H8C7.44772 10 7 10.4477 7 11V13C7 13.5523 7.44772 14 8 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn HorizontalDistribute(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M19 4V20M5 4V20M11 17H13C13.5523 17 14 16.5523 14 16V8C14 7.44772 13.5523 7 13 7H11C10.4477 7 10 7.44772 10 8V16C10 16.5523 10.4477 17 11 17Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignVerticalDown(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M4 20H20M6 16H9C9.55228 16 10 15.5523 10 15V9C10 8.44772 9.55228 8 9 8H6C5.44772 8 5 8.44772 5 9V15C5 15.5523 5.44772 16 6 16ZM15 16H18C18.5523 16 19 15.5523 19 15V5C19 4.44772 18.5523 4 18 4H15C14.4477 4 14 4.44772 14 5V15C14 15.5523 14.4477 16 15 16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignHorizontalLeft(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M4 4V20M9 10H15C15.5523 10 16 9.55228 16 9V5C16 4.44772 15.5523 4 15 4H9C8.44772 4 8 4.44772 8 5V9C8 9.55228 8.44772 10 9 10ZM9 20H19C19.5523 20 20 19.5523 20 19V15C20 14.4477 19.5523 14 19 14H9C8.44772 14 8 14.4477 8 15V19C8 19.5523 8.44772 20 9 20Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignRight(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "".to_string())]
    class: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M5 6H19M9 10H19M5 14H19M9 18H19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignHorizontalRight(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "none".to_string())]
    fill: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M20 4V20M9 10H15C15.5523 10 16 9.55228 16 9V5C16 4.44772 15.5523 4 15 4H9C8.44772 4 8 4.44772 8 5V9C8 9.55228 8.44772 10 9 10ZM5 20H15C15.5523 20 16 19.5523 16 19V15C16 14.4477 15.5523 14 15 14H5C4.44772 14 4 14.4477 4 15V19C4 19.5523 4.44772 20 5 20Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignCenter(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M5 6H19M7 10H17M5 14H19M7 18H17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignToRight(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "24".to_string())]
    height: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M20 5V19M4 12L16 12M16 12L13 9M16 12L13 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AlignVerticalCenter(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "".to_string())]
    class: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            width,
            height,
            view_box: "0 0 24 24",
            fill,
            path {
                d: "M10 12L14 12M4 12H5M19 12H20M6 16H9C9.55228 16 10 15.5523 10 15V9C10 8.44772 9.55228 8 9 8H6C5.44772 8 5 8.44772 5 9V15C5 15.5523 5.44772 16 6 16ZM15 18H18C18.5523 18 19 17.5523 19 17V7C19 6.44772 18.5523 6 18 6H15C14.4477 6 14 6.44772 14 7V17C14 17.5523 14.4477 18 15 18Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
