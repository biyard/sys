use dioxus::prelude::*;
#[component]
pub fn Headset(
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
                d: "M6 13H6.75C7.44036 13 8 13.5596 8 14.25V17.75C8 18.4404 7.44036 19 6.75 19H6C4.34315 19 3 17.6569 3 16C3 14.3431 4.34315 13 6 13ZM6 13V11C6 7.68629 8.68629 5 12 5C15.3137 5 18 7.68629 18 11V13M18 13H17.25C16.5596 13 16 13.5596 16 14.25V17.75C16 18.4404 16.5596 19 17.25 19H18C19.6569 19 21 17.6569 21 16C21 14.3431 19.6569 13 18 13Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Help(
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
                d: "M9 10C9 9.40666 9.17595 8.82664 9.50559 8.33329C9.83524 7.83994 10.3038 7.45543 10.852 7.22836C11.4001 7.0013 12.0033 6.94189 12.5853 7.05765C13.1672 7.1734 13.7018 7.45912 14.1213 7.87868C14.5409 8.29824 14.8266 8.83279 14.9424 9.41473C15.0581 9.99667 14.9987 10.5999 14.7716 11.1481C14.5446 11.6962 14.1601 12.1648 13.6667 12.4944C13.1734 12.8241 12.5933 13 12 13V14M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "17",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Warning(
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
                d: "M12 7V13M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "16.5",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn HelpQuestion(
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
                d: "M10 9C10 8.60444 10.1173 8.21776 10.3371 7.88886C10.5568 7.55996 10.8692 7.30362 11.2346 7.15224C11.6001 7.00087 12.0022 6.96126 12.3902 7.03843C12.7781 7.1156 13.1345 7.30608 13.4142 7.58579C13.6939 7.86549 13.8844 8.22186 13.9616 8.60982C14.0387 8.99778 13.9991 9.39992 13.8478 9.76537C13.6964 10.1308 13.44 10.4432 13.1111 10.6629C12.7822 10.8827 12.3956 11 12 11V12M14.25 19L12.8 20.9333C12.4 21.4667 11.6 21.4667 11.2 20.9333L9.75 19H7C4.79086 19 3 17.2091 3 15V7C3 4.79086 4.79086 3 7 3H17C19.2091 3 21 4.79086 21 7V15C21 17.2091 19.2091 19 17 19H14.25Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "15",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn HeadsetMic2Off(
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
            path {
                d: "M4 4L20 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.75921 4.67342C4.66139 5.97782 4 7.66167 4 9.5V11V17C4 17.5523 4.44772 18 5 18H7C8.65685 18 10 16.6569 10 15V13C10 11.3431 8.65685 10 7 10H6V9.5C6 8.21416 6.44125 7.03137 7.1806 6.09481L5.75921 4.67342ZM14.8362 10.922C15.3821 10.3537 16.1498 10 17 10H18V9.5C18 6.46243 15.5376 4 12.5 4H11.5C10.4721 4 9.51006 4.28198 8.68701 4.7728L7.24039 3.32618C8.45025 2.48986 9.91792 2 11.5 2H12.5C16.6421 2 20 5.35786 20 9.5V11V16.0858L18 14.0858V12H17C16.702 12 16.4345 12.1303 16.2513 12.3371L14.8362 10.922ZM14.0012 12.9154C14.0004 12.9435 14 12.9717 14 13V15C14 16.6569 15.3431 18 17 18H17.6126L17.5072 18.3162C17.3711 18.7246 16.9889 19 16.5585 19H13C12.4477 19 12 19.4477 12 20C12 20.5523 12.4477 21 13 21H16.5585C17.8498 21 18.9962 20.1737 19.4045 18.9487L19.562 18.4762L17.0858 16H17C16.4477 16 16 15.5523 16 15V14.9142L14.0012 12.9154ZM6 16V12H7C7.55228 12 8 12.4477 8 13V15C8 15.5523 7.55228 16 7 16H6Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn HeadsetMic2(
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
                d: "M19 11H17C15.8954 11 15 11.8954 15 13V15C15 16.1046 15.8954 17 17 17H19M19 11V17M19 11V9.5C19 5.91015 16.0899 3 12.5 3H11.5C7.91015 3 5 5.91015 5 9.5V11M19 17L18.4558 18.6325C18.1836 19.4491 17.4193 20 16.5585 20H13M5 11H7C8.10457 11 9 11.8954 9 13V15C9 16.1046 8.10457 17 7 17H5V11Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Support(
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
                d: "M18 6L14.8284 9.17157M14.8284 9.17157C14.1046 8.44772 13.1046 8 12 8C10.8954 8 9.89543 8.44772 9.17157 9.17157M14.8284 9.17157C15.5523 9.89543 16 10.8954 16 12C16 13.1046 15.5523 14.1046 14.8284 14.8284M18 18L14.8284 14.8284M14.8284 14.8284C14.1046 15.5523 13.1046 16 12 16C10.8954 16 9.89543 15.5523 9.17157 14.8284M6 18L9.17157 14.8284M9.17157 14.8284C8.44772 14.1046 8 13.1046 8 12C8 10.8954 8.44772 9.89543 9.17157 9.17157M6 6L9.17157 9.17157M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Info(
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
                d: "M12 11V16M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "7.5",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn HeadsetOff(
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
                d: "M5 5L21 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M6.84745 6.26167C5.70041 7.50834 5 9.17237 5 11V11.5C5 11.5113 5.00019 11.5226 5.00056 11.5339C5.00019 11.5451 5 11.5564 5 11.5677V12C5 12.042 5.00259 12.0834 5.00762 12.1241C3.27853 12.5655 2 14.1334 2 16C2 18.2091 3.79086 20 6 20H6.75C7.99264 20 9 18.9926 9 17.75V14.25C9 13.0918 8.12494 12.138 6.99991 12.0137L7 12V11.5677C7 11.5564 6.99981 11.5451 6.99944 11.5339C6.99981 11.5226 7 11.5113 7 11.5V11C7 9.72468 7.47747 8.56088 8.26339 7.67761L6.84745 6.26167ZM15.8796 12.4654C16.1969 12.2213 16.5812 12.06 17.0001 12.0137L17 12V11.5V11C17 8.23858 14.7614 6 12 6C11.2434 6 10.526 6.16807 9.88313 6.46891L8.40603 4.99182C9.45659 4.36204 10.686 4 12 4C15.866 4 19 7.13401 19 11V11.5V12C19 12.042 18.9974 12.0834 18.9924 12.1241C20.7215 12.5655 22 14.1334 22 16C22 16.7418 21.7981 17.4365 21.4462 18.032L19.9323 16.518C19.9764 16.3528 20 16.1792 20 16C20 14.8954 19.1046 14 18 14H17.4142L15.8796 12.4654ZM15 14.4142V17.75C15 18.9926 16.0074 20 17.25 20H18C18.7418 20 19.4365 19.7981 20.032 19.4462L18.518 17.9323C18.3528 17.9764 18.1792 18 18 18H17.25C17.1119 18 17 17.8881 17 17.75V16.4142L15 14.4142ZM6 14C4.89543 14 4 14.8954 4 16C4 17.1046 4.89543 18 6 18H6.75C6.88807 18 7 17.8881 7 17.75V14.25C7 14.1119 6.88807 14 6.75 14H6Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Help2(
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
                d: "M9 10C9 9.40666 9.17595 8.82664 9.50559 8.33329C9.83524 7.83994 10.3038 7.45543 10.852 7.22836C11.4001 7.0013 12.0033 6.94189 12.5853 7.05765C13.1672 7.1734 13.7018 7.45912 14.1213 7.87868C14.5409 8.29824 14.8266 8.83279 14.9424 9.41473C15.0581 9.99667 14.9987 10.5999 14.7716 11.1481C14.5446 11.6962 14.1601 12.1648 13.6667 12.4944C13.1734 12.8241 12.5933 13 12 13V14M5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3H5C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "17",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn HeadsetMic1(
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
                d: "M6 10H6.75C7.44036 10 8 10.5596 8 11.25V14.75C8 15.4404 7.44036 16 6.75 16H6C4.34315 16 3 14.6569 3 13C3 11.3431 4.34315 10 6 10ZM6 10V9C6 5.68629 8.68629 3 12 3C15.3137 3 18 5.68629 18 9V10M18 10H17.25C16.5596 10 16 10.5596 16 11.25V14.75C16 15.4404 16.5596 16 17.25 16H18M18 10C19.6569 10 21 11.3431 21 13C21 14.6569 19.6569 16 18 16M18 16L17.3787 18.4851C17.1561 19.3754 16.3562 20 15.4384 20H13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
