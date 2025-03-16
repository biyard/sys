use dioxus::prelude::*;
#[component]
pub fn LayerCheck(
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
                d: "M3 6C3 4.34315 4.34315 3 6 3H14C15.6569 3 17 4.34315 17 6V14C17 15.6569 15.6569 17 14 17H6C4.34315 17 3 15.6569 3 14V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 11L8.5 12.5L13 8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M21 7V18C21 19.6569 19.6569 21 18 21H7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BorderTop(
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
                cx: "20",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "1",
                fill: "black",
            }
            path {
                d: "M4 4H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "4",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn BorderBottom(
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
                d: "M4 20H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "20",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "16",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Layer1(
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
                d: "M20 10L12 5L4 10L12 15L20 10Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M20 14L12 19L4 14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BorderLeft(
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
            circle {
                cx: "20",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "12",
                r: "1",
                fill: "black",
            }
            path {
                d: "M4 4L4 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn BorderTopLeft(
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
            circle {
                cx: "20",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
                fill: "black",
            }
            path {
                d: "M4 20L4 5C4 4.44772 4.44772 4 5 4L20 4",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AddLayer1(
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
                d: "M3 10C3 8.34315 4.34315 7 6 7H14C15.6569 7 17 8.34315 17 10V18C17 19.6569 15.6569 21 14 21H6C4.34315 21 3 19.6569 3 18V10Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 14V11M10 14V17M10 14H13M10 14H7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 3L18 3C19.6569 3 21 4.34315 21 6L21 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BorderRight(
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
            circle {
                cx: "16",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
                fill: "black",
            }
            path {
                d: "M20 4L20 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn NoBorder(
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
                cx: "20",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "12",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "16",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn LayerOff1(
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
                d: "M4 4L20 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.32711 6.74132L3.47001 9.152C3.17763 9.33474 3.00001 9.65521 3.00001 10C3.00001 10.3448 3.17763 10.6653 3.47001 10.848L11.47 15.848C11.7943 16.0507 12.2057 16.0507 12.53 15.848L14.9323 14.3465L13.481 12.8952L12 13.8208L5.88681 10L8.77849 8.1927L7.32711 6.74132ZM15.2215 11.8073L18.1132 10L12 6.17925L10.5191 7.10484L9.06768 5.65346L11.47 4.152C11.7943 3.94933 12.2057 3.94933 12.53 4.152L20.53 9.152C20.8224 9.33474 21 9.65521 21 10C21 10.3448 20.8224 10.6653 20.53 10.848L16.6729 13.2587L15.2215 11.8073ZM15.9425 15.3567L12 17.8208L4.53001 13.152C4.06167 12.8593 3.44472 13.0017 3.15201 13.47C2.8593 13.9383 3.00168 14.5553 3.47001 14.848L11.47 19.848C11.7943 20.0507 12.2057 20.0507 12.53 19.848L17.3939 16.8081L15.9425 15.3567ZM19.1344 15.7202L17.6831 14.2688L19.47 13.152C19.9383 12.8593 20.5553 13.0017 20.848 13.47C21.1407 13.9383 20.9983 14.5553 20.53 14.848L19.1344 15.7202Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Layer(
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
                d: "M4 8L12 4L20 8L12 12L4 8Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 12L12 16L20 12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L12 20L20 16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AddLayer(
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
                d: "M3 6C3 4.34315 4.34315 3 6 3H14C15.6569 3 17 4.34315 17 6V14C17 15.6569 15.6569 17 14 17H6C4.34315 17 3 15.6569 3 14V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M21 7V18C21 19.6569 19.6569 21 18 21H7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 10V7M10 10V13M10 10H13M10 10H7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BorderInner(
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
                cx: "20",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "16",
                r: "1",
                fill: "black",
            }
            path {
                d: "M4 12H12M20 12H12M12 12V20M12 12V4",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ZoomOut(
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
                d: "M15 4V7C15 8.10457 15.8954 9 17 9H20M9 4V7C9 8.10457 8.10457 9 7 9H4M15 20V17C15 15.8954 15.8954 15 17 15H20M9 20V17C9 15.8954 8.10457 15 7 15H4",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BorderHorizontal(
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
                cx: "20",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "16",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "20",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "20",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "8",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "4",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "4",
                cy: "16",
                r: "1",
                fill: "black",
            }
            path {
                d: "M4 12H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ZoomIn(
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
                d: "M4 15V18C4 19.1046 4.89543 20 6 20H9M15.2173 20H18C19.1046 20 20 19.1046 20 18V15M20 9V6C20 4.89543 19.1046 4 18 4H15M4 9V6C4 4.89543 4.89543 4 6 4H9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LayerOff(
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
                d: "M4 4L20 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M6.51183 5.62604L3.55276 7.10557C3.21398 7.27496 2.99997 7.62123 2.99997 8C2.99997 8.37877 3.21398 8.72504 3.55276 8.89443L11.5528 12.8944C11.8343 13.0352 12.1657 13.0352 12.4472 12.8944L13.3359 12.4501L11.5355 10.6497L6.23604 8L8.00254 7.11675L6.51183 5.62604ZM13.7308 10.0166L17.7639 8L12 5.11803L9.88816 6.17394L8.39744 4.68323L11.5528 3.10557C11.8343 2.96481 12.1657 2.96481 12.4472 3.10557L20.4472 7.10557C20.786 7.27496 21 7.62123 21 8C21 8.37877 20.786 8.72504 20.4472 8.89443L15.2215 11.5073L13.7308 10.0166ZM14.5118 13.626L12 14.882L4.44719 11.1056C3.95321 10.8586 3.35254 11.0588 3.10555 11.5528C2.85856 12.0468 3.05878 12.6474 3.55276 12.8944L11.5528 16.8944C11.8343 17.0352 12.1657 17.0352 12.4472 16.8944L16.0025 15.1168L14.5118 13.626ZM17.8882 14.1739L16.3974 12.6832L19.5528 11.1056C20.0467 10.8586 20.6474 11.0588 20.8944 11.5528C21.1414 12.0468 20.9412 12.6474 20.4472 12.8944L17.8882 14.1739ZM17.1785 16.2927L12 18.882L4.44719 15.1056C3.95321 14.8586 3.35254 15.0588 3.10555 15.5528C2.85856 16.0468 3.05878 16.6474 3.55276 16.8944L11.5528 20.8944C11.8343 21.0352 12.1657 21.0352 12.4472 20.8944L18.6692 17.7834L17.1785 16.2927ZM20.5497 16.8355L19.0641 15.3499L19.5528 15.1056C20.0467 14.8586 20.6474 15.0588 20.8944 15.5528C21.1235 16.011 20.9678 16.5611 20.5497 16.8355Z",
                fill: "black",
            }
        }
    }
}
