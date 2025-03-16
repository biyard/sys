use dioxus::prelude::*;
#[component]
pub fn DollarCoin(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14.5 9.08333L14.3563 8.96356C13.9968 8.66403 13.5438 8.5 13.0759 8.5H10.75C9.7835 8.5 9 9.2835 9 10.25V10.25C9 11.2165 9.7835 12 10.75 12H13.25C14.2165 12 15 12.7835 15 13.75V13.75C15 14.7165 14.2165 15.5 13.25 15.5H10.412C9.8913 15.5 9.39114 15.2969 9.01782 14.934L9 14.9167",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 8L12 7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 17V16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LockPaymentCard(
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
                d: "M21 10.5V8C21 6.89543 20.1046 6 19 6H5C3.89543 6 3 6.89543 3 8V17C3 18.1046 3.89543 19 5 19H12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 16.4286C15 15.9552 15.3838 15.5714 15.8571 15.5714H20.1429C20.6162 15.5714 21 15.9552 21 16.4286V19.1429C21 19.6162 20.6162 20 20.1429 20H15.8571C15.3838 20 15 19.6162 15 19.1429V16.4286Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16.7143 14.2857C16.7143 13.5756 17.2899 13 18 13C18.7101 13 19.2857 13.5756 19.2857 14.2857V15.5714H16.7143V14.2857Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 10H20.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 15H9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Wallet(
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
                d: "M19 8V6C19 4.89543 18.1046 4 17 4H6C4.89543 4 4 4.89543 4 6V18C4 19.1046 4.89543 20 6 20H17C18.1046 20 19 19.1046 19 18V16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "13",
                y: "8",
                width: "8",
                height: "8",
                rx: "1",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "17",
                cy: "12",
                r: "1.5",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn AddPaymentCard(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    height: String,
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
                d: "M21 12.5V8C21 6.89543 20.1046 6 19 6H5C3.89543 6 3 6.89543 3 8V17C3 18.1046 3.89543 19 5 19H13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.5 15V17.5M18.5 20V17.5M18.5 17.5H16M18.5 17.5H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 10H20.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 15H9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Money(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "none".to_string())]
    fill: String,
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
            g { clip_path: "url(#clip0_0_1537)",
                rect {
                    x: "2",
                    y: "6",
                    width: "20",
                    height: "12",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M22 10C21.4747 10 20.9546 9.89654 20.4693 9.69552C19.984 9.4945 19.543 9.19986 19.1716 8.82843C18.8001 8.45699 18.5055 8.01604 18.3045 7.53073C18.1035 7.04543 18 6.52529 18 6L22 6L22 10Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M18 18C18 16.9391 18.4214 15.9217 19.1716 15.1716C19.9217 14.4214 20.9391 14 22 14L22 18L18 18Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M2 14C3.06087 14 4.07828 14.4214 4.82843 15.1716C5.57857 15.9217 6 16.9391 6 18L2 18L2 14Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M6 6C6 7.06087 5.57857 8.07828 4.82843 8.82843C4.07828 9.57857 3.06087 10 2 10L2 6H6Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                circle {
                    cx: "12",
                    cy: "12",
                    r: "2",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
            defs {
                clipPath { id: "clip0_0_1537",
                    rect { width: "24", height: "24", fill: "white" }
                }
            }
        }
    }
}
#[component]
pub fn Dollar(
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
                d: "M16.1667 7.41667L15.8159 7.14105C15.2873 6.72575 14.6346 6.5 13.9624 6.5H9.75C8.23122 6.5 7 7.73122 7 9.25V9.25C7 10.7688 8.23122 12 9.75 12H14.25C15.7688 12 17 13.2312 17 14.75V14.75C17 16.2688 15.7688 17.5 14.25 17.5H9.16695C8.41638 17.5 7.69307 17.2186 7.13978 16.7115L7 16.5833",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 19L12 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Rupee(
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
                d: "M7 5H12C14.2091 5 16 6.79086 16 9V9C16 11.2091 14.2091 13 12 13H9L15 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 5L18 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 9L18 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Tag(
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
                d: "M11.1716 3H5C3.89543 3 3 3.89543 3 5V11.1716C3 11.702 3.21071 12.2107 3.58579 12.5858L10.8787 19.8787C12.0503 21.0503 13.9497 21.0503 15.1213 19.8787L19.8787 15.1213C21.0503 13.9497 21.0503 12.0503 19.8787 10.8787L12.5858 3.58579C12.2107 3.21071 11.702 3 11.1716 3Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "7.5",
                cy: "7.5",
                r: "1.5",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn MoneyDollar(
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
            g { clip_path: "url(#clip0_0_1544)",
                rect {
                    x: "2",
                    y: "6",
                    width: "20",
                    height: "12",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M22 10C21.4747 10 20.9546 9.89654 20.4693 9.69552C19.984 9.4945 19.543 9.19986 19.1716 8.82843C18.8001 8.45699 18.5055 8.01604 18.3045 7.53073C18.1035 7.04543 18 6.52529 18 6L22 6L22 10Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M18 18C18 16.9391 18.4214 15.9217 19.1716 15.1716C19.9217 14.4214 20.9391 14 22 14L22 18L18 18Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M2 14C3.06087 14 4.07828 14.4214 4.82843 15.1716C5.57857 15.9217 6 16.9391 6 18L2 18L2 14Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M6 6C6 7.06087 5.57857 8.07828 4.82843 8.82843C4.07828 9.57857 3.06087 10 2 10L2 6H6Z",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M14.0741 9.5H11.3333C10.597 9.5 10 10.0596 10 10.75C10 11.4404 10.597 12 11.3333 12H13.1111C13.8475 12 14.4444 12.5596 14.4444 13.25C14.4444 13.9404 13.8475 14.5 13.1111 14.5H10",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M12 9.51733V8.5",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M12 15.5173V14.5",
                    stroke: "black",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
            defs {
                clipPath { id: "clip0_0_1544",
                    rect { width: "24", height: "24", fill: "white" }
                }
            }
        }
    }
}
#[component]
pub fn RemovePaymentCard(
    #[props(default = "24".to_string())]
    width: String,
    #[props(default = "".to_string())]
    class: String,
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
                d: "M21 11.5V8C21 6.89543 20.1046 6 19 6H5C3.89543 6 3 6.89543 3 8V17C3 18.1046 3.89543 19 5 19H12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.8787 14.8787C15.3358 15.4216 15 16.1716 15 17C15 18.6569 16.3431 20 18 20C18.8284 20 19.5784 19.6642 20.1213 19.1213M15.8787 14.8787C16.4216 14.3358 17.1716 14 18 14C19.6569 14 21 15.3431 21 17C21 17.8284 20.6642 18.5784 20.1213 19.1213M15.8787 14.8787L18 17L20.1213 19.1213",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M3 10H20.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 15H9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn WonCoin(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 9V16L12 9.77778L16 16V9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 12C5.44772 12 5 12.4477 5 13C5 13.5523 5.44772 14 6 14V12ZM9 14H10V12H9V14ZM6 14H9V12H6V14Z",
                fill: "black",
            }
            path {
                d: "M15 14L14 14L14 12L15 12L15 14ZM18 12C18.5523 12 19 12.4477 19 13C19 13.5523 18.5523 14 18 14L18 12ZM15 12L18 12L18 14L15 14L15 12Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Bitcoin(
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
                d: "M7 7V12V17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 12H14.5C15.8807 12 17 13.1193 17 14.5V14.5C17 15.8807 15.8807 17 14.5 17H7V12Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 7H14.5C15.8807 7 17 8.11929 17 9.5V9.5C17 10.8807 15.8807 12 14.5 12H7V7Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 7L9 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 19L9 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M13 7L13 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M13 19L13 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn PaymentCard(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    height: String,
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
            rect {
                x: "3",
                y: "6",
                width: "18",
                height: "13",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 10H20.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 15H9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FavouritePaymentCard(
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
                d: "M21 11.5V8C21 6.89543 20.1046 6 19 6H5C3.89543 6 3 6.89543 3 8V17C3 18.1046 3.89543 19 5 19H13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 10H20.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.5147 14.5147C14.8284 15.201 14.8284 16.3137 15.5147 17L18 19.4853L20.4853 17.0001C21.1716 16.3138 21.1716 15.2011 20.4853 14.5148C19.799 13.8285 18.6863 13.8285 18 14.5147C17.3138 13.8285 16.201 13.8284 15.5147 14.5147Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 15H9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn RupeeCoin(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "none".to_string())]
    fill: String,
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 8H12C13.1046 8 14 8.89543 14 10V11.1429C14 12.2474 13.1046 13.1429 12 13.1429H9.33333L13.3333 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 8L16 8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 10.5718L16 10.5718",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BotcoinCoin(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 12H14.25C15.2165 12 16 12.7835 16 13.75V13.75C16 14.7166 15.2165 15.5001 14.25 15.5001H9V12Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 8.5H14.25C15.2165 8.5 16 9.28352 16 10.25V10.25C16 11.2166 15.2165 12.0001 14.25 12.0001H9V8.5Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M11 8L11 7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M11 17L11 16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 8L14 7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 17L14 16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
