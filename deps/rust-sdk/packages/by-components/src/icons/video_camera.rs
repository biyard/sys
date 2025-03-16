use dioxus::prelude::*;
#[component]
pub fn VideoPlaylist(
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
                d: "M9 12V8L12.1429 10L9 12Z",
                fill: "black",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Video(
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
                d: "M4 15V9C4 7.89543 4.89543 7 6 7H13C14.1046 7 15 7.89543 15 9V15C15 16.1046 14.1046 17 13 17H6C4.89543 17 4 16.1046 4 15Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.3753 8.29976L15.3753 10.6998C15.1381 10.8895 15 11.1768 15 11.4806V12.5194C15 12.8232 15.1381 13.1105 15.3753 13.3002L18.3753 15.7002C19.0301 16.2241 20 15.7579 20 14.9194V9.08062C20 8.24212 19.0301 7.77595 18.3753 8.29976Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AddPhotoCamera(
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
                d: "M21 11V18C21 19.1046 20.1046 20 19 20H5C3.89543 20 3 19.1046 3 18V9C3 7.89543 3.89543 7 5 7H6.5C7.12951 7 7.72229 6.70361 8.1 6.2L9.15 4.8C9.52771 4.29639 10.1205 4 10.75 4H13.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.5 4V6.5M18.5 9V6.5M18.5 6.5H16M18.5 6.5H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "13",
                r: "4",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VideoOff(
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
                d: "M4 4L20 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.28578 6H6C4.34315 6 3 7.34315 3 9V15C3 16.6569 4.34315 18 6 18H13C14.6302 18 15.9567 16.6998 15.999 15.0798L17.7506 16.4811C17.7737 16.4996 17.797 16.5175 17.8205 16.5347L14 12.7142V15C14 15.5523 13.5523 16 13 16H6C5.44772 16 5 15.5523 5 15V9C5 8.44772 5.44772 8 6 8H9.28578L7.28578 6ZM14 9.88579V9C14 8.44772 13.5523 8 13 8H12.1142L10.1142 6H13C14.6302 6 15.9567 7.30024 15.999 8.92021L17.7506 7.51889C19.0601 6.47127 21 7.40361 21 9.08062V14.9194C21 15.4871 20.7777 15.9695 20.4328 16.3186L19 14.8858V9.08063L16 11.4806V11.8858L14 9.88579Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn PhotoCamera(
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
                d: "M3 9C3 7.89543 3.89543 7 5 7H6.5C7.12951 7 7.72229 6.70361 8.1 6.2L9.15 4.8C9.52771 4.29639 10.1205 4 10.75 4H13.25C13.8795 4 14.4723 4.29639 14.85 4.8L15.9 6.2C16.2777 6.70361 16.8705 7 17.5 7H19C20.1046 7 21 7.89543 21 9V18C21 19.1046 20.1046 20 19 20H5C3.89543 20 3 19.1046 3 18V9Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "13",
                r: "4",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn PhotoCameraOff(
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
                d: "M6.96899 5.8832C6.82644 5.9589 6.66577 6 6.5 6H5C3.34315 6 2 7.34315 2 9V18C2 19.6569 3.34315 21 5 21H19C19.9023 21 20.7115 20.6017 21.2615 19.9713C21.0898 19.9293 20.927 19.8412 20.7929 19.7071L19.7487 18.6629C19.5655 18.8697 19.298 19 19 19H5C4.44772 19 4 18.5523 4 18V9C4 8.44772 4.44772 8 5 8H6.5C7.20173 8 7.87302 7.75448 8.4041 7.31832L6.96899 5.8832ZM9.66393 8.57815C8.07949 9.41694 7 11.0824 7 13C7 15.7614 9.23858 18 12 18C13.9176 18 15.5831 16.9205 16.4219 15.3361L14.8908 13.805C14.539 15.0709 13.378 16 12 16C10.3431 16 9 14.6569 9 13C9 11.622 9.92908 10.461 11.195 10.1092L9.66393 8.57815ZM16.9993 13.0851L11.9149 8.00071C11.9432 8.00024 11.9716 8 12 8C14.7614 8 17 10.2386 17 13C17 13.0284 16.9998 13.0568 16.9993 13.0851ZM20 16.0858V9C20 8.44772 19.5523 8 19 8H17.5C16.5557 8 15.6666 7.55542 15.1 6.8L14.05 5.4C13.8611 5.14819 13.5648 5 13.25 5H10.75C10.4352 5 10.1389 5.14819 9.95 5.4L9.67752 5.76331L8.24895 4.33474L8.35 4.2C8.91656 3.44458 9.80573 3 10.75 3H13.25C14.1943 3 15.0834 3.44458 15.65 4.2L16.7 5.6C16.8889 5.85181 17.1852 6 17.5 6H19C20.6569 6 22 7.34315 22 9V18C22 18.0283 21.9996 18.0565 21.9988 18.0846L20 16.0858Z",
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
