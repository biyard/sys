use dioxus::prelude::*;
#[component]
pub fn Suitcase2(
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
                d: "M9 6C9 4.89543 9.89543 4 11 4H13C14.1046 4 15 4.89543 15 6V8H9V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "4",
                y: "8",
                width: "16",
                height: "11",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn SuitcaseOff(
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
                d: "M4 4L21 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M8 6.91421V7H6C4.34315 7 3 8.34315 3 10V17C3 18.6569 4.34315 20 6 20H18C18.8502 20 19.6179 19.6463 20.1638 19.078L18.7487 17.6629C18.5655 17.8697 18.298 18 18 18H6C5.44772 18 5 17.5523 5 17V10C5 9.44772 5.44772 9 6 9H9H10.0858L8 6.91421ZM19 15.0858V10C19 9.44772 18.5523 9 18 9H15H12.9142L10.9142 7H14V6C14 5.44772 13.5523 5 13 5H11C10.4477 5 10 5.44772 10 6V6.08579L8.40607 4.49186C8.92594 3.59965 9.89294 3 11 3H13C14.6569 3 16 4.34315 16 6V7H18C19.6569 7 21 8.34315 21 10V17C21 17.0283 20.9996 17.0565 20.9988 17.0846L19 15.0858Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Luggage(
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
            rect {
                x: "6",
                y: "7",
                width: "12",
                height: "13",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 11V16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 11V16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16 20L16 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20L8 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 4C10 3.44772 10.4477 3 11 3H13C13.5523 3 14 3.44772 14 4V7H10V4Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Suitcase(
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
            path {
                d: "M3 10C3 8.89543 3.89543 8 5 8H19C20.1046 8 21 8.89543 21 10V12C21 12.5523 20.5523 13 20 13H4C3.44772 13 3 12.5523 3 12V10Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 6C9 4.89543 9.89543 4 11 4H13C14.1046 4 15 4.89543 15 6V8H9V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 13H20V18C20 19.1046 19.1046 20 18 20H6C4.89543 20 4 19.1046 4 18V13Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 12V14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Suitcase1(
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
                d: "M4 10C4 8.89543 4.89543 8 6 8H18C19.1046 8 20 8.89543 20 10V10C20 11.6569 18.6569 13 17 13H7C5.34315 13 4 11.6569 4 10V10Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 6C9 4.89543 9.89543 4 11 4H13C14.1046 4 15 4.89543 15 6V8H9V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "4",
                y: "8",
                width: "16",
                height: "12",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 12V14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 12V14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
