use dioxus::prelude::*;
#[component]
pub fn VolumeLow(
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
                d: "M7 14V10C7 9.44772 7.44772 9 8 9H10.6492C10.8763 9 11.0966 8.92272 11.2739 8.78087L14.3753 6.29976C15.0301 5.77595 16 6.24212 16 7.08062V16.9194C16 17.7579 15.0301 18.2241 14.3753 17.7002L11.2739 15.2191C11.0966 15.0773 10.8763 15 10.6492 15H8C7.44772 15 7 14.5523 7 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VolumeDown(
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
                d: "M5 14V10C5 9.44772 5.44772 9 6 9H8.64922C8.87629 9 9.0966 8.92272 9.27391 8.78087L12.3753 6.29976C13.0301 5.77595 14 6.24212 14 7.08062V16.9194C14 17.7579 13.0301 18.2241 12.3753 17.7002L9.27391 15.2191C9.0966 15.0773 8.87629 15 8.64922 15H6C5.44772 15 5 14.5523 5 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.8302 15.2139C18.5435 14.3639 18.9537 13.3008 18.9963 12.1919C19.0389 11.0831 18.7114 9.99163 18.0655 9.08939",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VolumeDown1(
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
                d: "M3 14V10C3 9.44772 3.44772 9 4 9H6.64922C6.87629 9 7.0966 8.92272 7.27391 8.78087L10.3753 6.29976C11.0301 5.77595 12 6.24212 12 7.08062V16.9194C12 17.7579 11.0301 18.2241 10.3753 17.7002L7.27391 15.2191C7.0966 15.0773 6.87629 15 6.64922 15H4C3.44772 15 3 14.5523 3 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16 12H18.5H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VolumeUp1(
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
                d: "M3 14V10C3 9.44772 3.44772 9 4 9H6.64922C6.87629 9 7.0966 8.92272 7.27391 8.78087L10.3753 6.29976C11.0301 5.77595 12 6.24212 12 7.08062V16.9194C12 17.7579 11.0301 18.2241 10.3753 17.7002L7.27391 15.2191C7.0966 15.0773 6.87629 15 6.64922 15H4C3.44772 15 3 14.5523 3 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.5 9.5V12M18.5 14.5V12M18.5 12H16M18.5 12H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn MicOff(
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
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M8.99998 7.91421V11C8.99998 12.6569 10.3431 14 12 14C12.8502 14 13.6179 13.6463 14.1638 13.078L12.7487 11.6629C12.5655 11.8697 12.298 12 12 12C11.4477 12 11 11.5523 11 11V9.91422L8.99998 7.91421ZM13 9.08579V5C13 4.44772 12.5523 4 12 4C11.4477 4 11 4.44772 11 5V7.08579L8.99998 5.08579V5C8.99998 3.34315 10.3431 2 12 2C13.6568 2 15 3.34315 15 5V11C15 11.0283 14.9996 11.0565 14.9988 11.0846L13 9.08579ZM15.5782 14.4924C15.4022 14.6727 15.2121 14.8402 15.0091 14.9932C14.1658 15.6286 13.143 15.9808 12.0872 15.9992C12.0593 15.9997 12.0314 16 12.0036 16C10.9769 16.0007 9.97418 15.6854 9.1321 15.0958C8.26716 14.4901 7.61616 13.6262 7.27239 12.6278C7.09258 12.1056 6.5235 11.8281 6.0013 12.0079C5.47911 12.1877 5.20155 12.7568 5.38135 13.279C5.86263 14.6767 6.77403 15.8862 7.98495 16.7341C8.88688 17.3656 9.92048 17.7724 11 17.9282V20H8.99998C8.4477 20 7.99998 20.4477 7.99998 21C7.99998 21.5523 8.4477 22 8.99998 22H12H15C15.5523 22 16 21.5523 16 21C16 20.4477 15.5523 20 15 20H13V17.9282C14.1617 17.7605 15.2677 17.3025 16.2127 16.5904C16.4904 16.3812 16.7509 16.1525 16.9924 15.9067L15.5782 14.4924ZM18.1875 14.2733L16.6784 12.7642C16.7159 12.6648 16.7503 12.5639 16.7815 12.4619C16.943 11.9337 17.502 11.6365 18.0302 11.7979C18.5583 11.9594 18.8556 12.5184 18.6941 13.0466C18.5638 13.4729 18.3938 13.8834 18.1875 14.2733Z",
                fill: "black",
            }
            path {
                d: "M5 5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VolumeOff(
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
                d: "M5 5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M8.00287 6.91705L6.64922 7.99997H4C2.89543 7.99997 2 8.8954 2 9.99997V14C2 15.1045 2.89543 16 4 16H6.64922L9.75061 18.4811C11.0601 19.5287 13 18.5964 13 16.9193V11.9142L11 9.91419V16.9193L7.89861 14.4382C7.54398 14.1545 7.10336 14 6.64922 14H4V9.99997H6.64922C7.10336 9.99997 7.54398 9.84541 7.89861 9.56171L9.42578 8.33997L8.00287 6.91705ZM11 7.08576V7.0806L10.9971 7.08289L9.57422 5.65998L9.75061 5.51886C11.0601 4.47124 13 5.40358 13 7.0806V9.08576L11 7.08576ZM15.3187 14.2329C15.2402 14.3496 15.1553 14.4626 15.0642 14.5711C14.7092 14.9942 14.7644 15.625 15.1874 15.98C15.6105 16.335 16.2413 16.2798 16.5963 15.8567C16.6491 15.7937 16.7006 15.7297 16.7507 15.6648L15.3187 14.2329ZM17.7214 13.8071L15.9991 12.0849C16.0174 11.2215 15.7558 10.3746 15.2524 9.67148C14.9309 9.22242 15.0343 8.59776 15.4834 8.27626C15.9324 7.95476 16.5571 8.05817 16.8786 8.50724C17.6537 9.58993 18.0467 10.8997 17.9956 12.2303C17.9749 12.7694 17.8818 13.2994 17.7214 13.8071ZM18.1736 17.0878C18.1586 17.106 18.1435 17.1242 18.1284 17.1423C17.7734 17.5654 17.8285 18.1961 18.2516 18.5511C18.6479 18.8836 19.2263 18.8563 19.5897 18.5039L18.1736 17.0878ZM20.752 16.8377L19.2646 15.3504C19.7379 14.3242 19.9918 13.2019 19.9998 12.0558C20.0128 10.1942 19.3762 8.3864 18.1996 6.94374C17.8505 6.51575 17.9145 5.88583 18.3425 5.53677C18.7705 5.18771 19.4004 5.25169 19.7494 5.67968C21.2202 7.48301 22.016 9.74281 21.9998 12.0698C21.988 13.7484 21.5543 15.3862 20.752 16.8377Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Mic(
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
                d: "M12 17V21M12 21H9M12 21H15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "10",
                y: "3",
                width: "4",
                height: "10",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.7378 12.7542C17.3674 13.9659 16.6228 15.0293 15.6109 15.7918C14.599 16.5544 13.3716 16.977 12.1047 16.9991C10.8378 17.0212 9.59647 16.6417 8.55854 15.9149C7.52061 15.1881 6.73941 14.1515 6.32689 12.9534",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VolumeUp(
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
                d: "M3 14V10C3 9.44772 3.44772 9 4 9H6.64922C6.87629 9 7.0966 8.92272 7.27391 8.78087L10.3753 6.29976C11.0301 5.77595 12 6.24212 12 7.08062V16.9194C12 17.7579 11.0301 18.2241 10.3753 17.7002L7.27391 15.2191C7.0966 15.0773 6.87629 15 6.64922 15H4C3.44772 15 3 14.5523 3 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.8302 15.2139C16.5435 14.3639 16.9537 13.3008 16.9963 12.1919C17.0389 11.0831 16.7114 9.99163 16.0655 9.08939",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.8944 17.7851C20.2406 16.1807 20.9852 14.1571 20.9998 12.0628C21.0144 9.96855 20.2982 7.93473 18.9745 6.31174",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn VolumeMute(
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
                d: "M3 14V10C3 9.44772 3.44772 9 4 9H6.64922C6.87629 9 7.0966 8.92272 7.27391 8.78087L10.3753 6.29976C11.0301 5.77595 12 6.24212 12 7.08062V16.9194C12 17.7579 11.0301 18.2241 10.3753 17.7002L7.27391 15.2191C7.0966 15.0773 6.87629 15 6.64922 15H4C3.44772 15 3 14.5523 3 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16 9.5L18.5 12M21 14.5L18.5 12M18.5 12L21 9.5M18.5 12L16 14.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
