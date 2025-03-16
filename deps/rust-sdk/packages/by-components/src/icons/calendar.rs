use dioxus::prelude::*;
#[component]
pub fn CalendarStarred(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "none".to_string())]
    fill: String,
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10H4M20 10V11.75M4 10V19C4 20.1046 4.89543 21 6 21H11M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M16.723 13.666C16.8255 13.4196 17.1745 13.4196 17.277 13.666L18.2521 16.0105C18.2953 16.1144 18.393 16.1854 18.5052 16.1944L21.0363 16.3973C21.3023 16.4186 21.4102 16.7506 21.2075 16.9242L19.279 18.5761C19.1936 18.6493 19.1563 18.7641 19.1824 18.8735L19.7716 21.3435C19.8335 21.6031 19.5511 21.8082 19.3234 21.6691L17.1564 20.3455C17.0604 20.2869 16.9396 20.2869 16.8436 20.3455L14.6766 21.6691C14.4489 21.8082 14.1665 21.6031 14.2284 21.3435L14.8176 18.8735C14.8437 18.7641 14.8064 18.6493 14.721 18.5761L12.7925 16.9242C12.5898 16.7506 12.6977 16.4186 12.9637 16.3973L15.4948 16.1944C15.607 16.1854 15.7047 16.1144 15.7479 16.0105L16.723 13.666Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn CalendarNote(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    height: String,
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10V19C20 20.1046 19.1046 21 18 21H6C4.89543 21 4 20.1046 4 19V10M20 10H4M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            line {
                x1: "8",
                y1: "14",
                x2: "16",
                y2: "14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            line {
                x1: "8",
                y1: "17",
                x2: "11",
                y2: "17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn Calendar(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10V19C20 20.1046 19.1046 21 18 21H6C4.89543 21 4 20.1046 4 19V10M20 10H4M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn CalendarBlock(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10H4M20 10V11.75M4 10V19C4 20.1046 4.89543 21 6 21H12M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M15.0251 15.0251C14.3918 15.6585 14 16.5335 14 17.5C14 19.433 15.567 21 17.5 21C18.4665 21 19.3415 20.6082 19.9749 19.9749M15.0251 15.0251C15.6585 14.3918 16.5335 14 17.5 14C19.433 14 21 15.567 21 17.5C21 18.4665 20.6082 19.3415 19.9749 19.9749M15.0251 15.0251L17.5 17.5L19.9749 19.9749",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn CalendarDownload(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10H4M20 10V11.75M4 10V19C4 20.1046 4.89543 21 6 21H12M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M18 15V21M18 21L15.5 18.5M18 21L20.5 18.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CalendarToday(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10V19C20 20.1046 19.1046 21 18 21H6C4.89543 21 4 20.1046 4 19V10M20 10H4M8 3V7M16 3C16 3 16 5.4379 16 7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            rect {
                x: "6",
                y: "12",
                width: "5",
                height: "5",
                rx: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn CalendarLock(
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
            rect {
                x: "13",
                y: "15",
                width: "8",
                height: "6",
                rx: "1",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M15 13C15 11.8954 15.8954 11 17 11V11C18.1046 11 19 11.8954 19 13V15H15V13Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9 3C9 2.44772 8.55228 2 8 2C7.44772 2 7 2.44772 7 3V4H6C4.34315 4 3 5.34315 3 7V10V19C3 20.6569 4.34315 22 6 22H9C9.55228 22 10 21.5523 10 21C10 20.4477 9.55228 20 9 20H6C5.44772 20 5 19.5523 5 19V11H12.9678C13.3941 10.1421 14.0862 9.43949 14.9364 9H5V7C5 6.44772 5.44772 6 6 6H7V7C7 7.55228 7.44772 8 8 8C8.55228 8 9 7.55228 9 7V6H15V7C15 7.55228 15.4477 8 16 8C16.5523 8 17 7.55228 17 7V6H18C18.5523 6 19 6.44772 19 7V8.96776C19.7399 9.33547 20.3643 9.90084 20.804 10.5948C20.9271 10.4286 21 10.2228 21 10V7C21 5.34315 19.6569 4 18 4H17V3C17 2.44772 16.5523 2 16 2C15.4477 2 15 2.44772 15 3V4H9V3Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn CalendarUpload(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10H4M20 10V11.75M4 10V19C4 20.1046 4.89543 21 6 21H12M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M18 21L18 15M18 15L20.5 17.5M18 15L15.5 17.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CalendarDelete(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    height: String,
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10H4M20 10V10.75M4 10V19C4 20.1046 4.89543 21 6 21H12M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M21 15L18.5 17.5M16 20L18.5 17.5M18.5 17.5L21 20M18.5 17.5L16 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn CalendarDay(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "none".to_string())]
    fill: String,
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10V19C20 20.1046 19.1046 21 18 21H6C4.89543 21 4 20.1046 4 19V10M20 10H4M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            rect {
                x: "13",
                y: "14",
                width: "5",
                height: "5",
                rx: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn DateRange(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10V19C20 20.1046 19.1046 21 18 21H6C4.89543 21 4 20.1046 4 19V10M20 10H4M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            rect {
                x: "6",
                y: "12",
                width: "3",
                height: "3",
                rx: "0.5",
                fill: "black",
            }
            rect {
                x: "10.5",
                y: "12",
                width: "3",
                height: "3",
                rx: "0.5",
                fill: "black",
            }
            rect {
                x: "15",
                y: "12",
                width: "3",
                height: "3",
                rx: "0.5",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn CalendarAdd(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10H4M20 10V10.75M4 10V19C4 20.1046 4.89543 21 6 21H12M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M17.5355 14V17.5355M17.5355 21.0711V17.5355M17.5355 17.5355L21.0711 17.5355M17.5355 17.5355H14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn CalendarCheck(
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
                d: "M20 10V7C20 5.89543 19.1046 5 18 5H6C4.89543 5 4 5.89543 4 7V10M20 10H4M20 10V11.75M4 10V19C4 20.1046 4.89543 21 6 21H11M8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M14 19L16 21L21 16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CalendarOff(
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
                d: "M3 3L21 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.19501 4.10923C3.92908 4.46099 3 5.622 3 7V10V19C3 20.6569 4.34315 22 6 22H18C19.378 22 20.539 21.0709 20.8908 19.805L19 17.9142V19C19 19.5523 18.5523 20 18 20H6C5.44772 20 5 19.5523 5 19V11H12.0858L10.0858 9H5V7C5 6.44772 5.44772 6 6 6H7V7C7 7.55228 7.44772 8 8 8C8.29797 8 8.5655 7.86968 8.74871 7.66292L5.19501 4.10923ZM12.9142 9H19V7C19 6.44772 18.5523 6 18 6H17V7C17 7.55228 16.5523 8 16 8C15.4477 8 15 7.55228 15 7V6H9.91421L7 3.08579V3C7 2.44772 7.44772 2 8 2C8.55228 2 9 2.44772 9 3V4H15V3C15 2.44772 15.4477 2 16 2C16.5523 2 17 2.44772 17 3V4H18C19.6569 4 21 5.34315 21 7V10V17.0858L19 15.0858V11H14.9142L12.9142 9Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn CalendarSetting(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
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
                d: "M4 10V19C4 20.1046 4.89543 21 6 21H11M4 10V7C4 5.89543 4.89543 5 6 5H18C19.1046 5 20 5.89543 20 7V10H4ZM8 3V7M16 3V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.4641 15C18.2043 15 18.8506 15.4021 19.1964 15.9998M17.4641 15C16.7239 15 16.0776 15.4021 15.7318 15.9998M17.4641 15V13M17.4641 21V18.9667M20.9282 15L19.1964 15.9998M14 19L15.7318 18.0002M20.9282 19L19.1965 18.0002M14 15L15.7318 15.9998M15.7318 18.0002C15.5615 17.706 15.4641 17.3644 15.4641 17C15.4641 16.6356 15.5615 16.294 15.7318 15.9998M15.7318 18.0002C16.1255 18.6807 16.7977 18.9802 17.4641 18.9667M17.4641 18.9667C18.1521 18.9529 18.8338 18.6057 19.1965 18.0002M19.1965 18.0002C19.3645 17.7197 19.4641 17.3838 19.4641 17C19.4641 16.6356 19.3667 16.294 19.1964 15.9998",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
