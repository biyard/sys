use dioxus::prelude::*;
#[component]
pub fn ChargingBattery(
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
                d: "M7.75 16H5C3.89543 16 3 15.1046 3 14V10C3 8.89543 3.89543 8 5 8H5.75M15.25 16H16C17.1046 16 18 15.1046 18 14V10C18 8.89543 17.1046 8 16 8H13.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M21 11V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 8L8 12H13L11 16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Battery(
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
            rect {
                x: "3",
                y: "8",
                width: "15",
                height: "8",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M21 11V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Bookmark(
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
                d: "M7 7.2C7 6.07989 7 5.51984 7.21799 5.09202C7.40973 4.71569 7.71569 4.40973 8.09202 4.21799C8.51984 4 9.07989 4 10.2 4H13.8C14.9201 4 15.4802 4 15.908 4.21799C16.2843 4.40973 16.5903 4.71569 16.782 5.09202C17 5.51984 17 6.07989 17 7.2V20L14.126 17.4453C13.3737 16.7766 12.9976 16.4423 12.5732 16.3154C12.1992 16.2035 11.8008 16.2035 11.4268 16.3154C11.0024 16.4423 10.6263 16.7766 9.87404 17.4453L7 20V7.2Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Save(
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
                d: "M4 6C4 4.89543 4.89543 4 6 4H12H14.1716C14.702 4 15.2107 4.21071 15.5858 4.58579L19.4142 8.41421C19.7893 8.78929 20 9.29799 20 9.82843V12V18C20 19.1046 19.1046 20 18 20H6C4.89543 20 4 19.1046 4 18V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 4H13V7C13 7.55228 12.5523 8 12 8H9C8.44772 8 8 7.55228 8 7V4Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 15C7 13.8954 7.89543 13 9 13H15C16.1046 13 17 13.8954 17 15V20H7V15Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn PrinterOff(
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
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M6.14912 5.06333C6.05235 5.35805 6 5.67291 6 6V7H5C3.34315 7 2 8.34315 2 10V17C2 17.5523 2.44772 18 3 18H6C6 19.6569 7.34315 21 9 21H15C16.6569 21 18 19.6569 18 18H19.0858L14.0858 13H7C6.44772 13 6 13.4477 6 14V16H4V10C4 9.44772 4.44772 9 5 9H7H10.0858L8.08579 7H8V6.91421L6.14912 5.06333ZM10.9142 7H16V6C16 5.44772 15.5523 5 15 5H9C8.97224 5 8.94475 5.00113 8.91756 5.00335L7.38535 3.47114C7.85149 3.17289 8.40555 3 9 3H15C16.6569 3 18 4.34315 18 6V7H19C20.6569 7 22 8.34315 22 10V17C22 17.298 21.8697 17.5655 21.6629 17.7487L19.9142 16H20V10C20 9.44772 19.5523 9 19 9H17H12.9142L10.9142 7ZM15.9102 11.996L15.004 11.0898C15.0013 11.0602 15 11.0303 15 11C15 10.4477 15.4477 10 16 10H18C18.5523 10 19 10.4477 19 11C19 11.5523 18.5523 12 18 12H16C15.9697 12 15.9398 11.9987 15.9102 11.996ZM8 18V15H16V18C16 18.5523 15.5523 19 15 19H9C8.44772 19 8 18.5523 8 18Z",
                fill: "black",
            }
            path {
                d: "M3 3L21 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Printer(
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
                d: "M7 6C7 4.89543 7.89543 4 9 4H15C16.1046 4 17 4.89543 17 6V8H7V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 14H17V18C17 19.1046 16.1046 20 15 20H9C7.89543 20 7 19.1046 7 18V14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18 17H21V10C21 8.89543 20.1046 8 19 8H5C3.89543 8 3 8.89543 3 10V17H6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18 11L16 11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
