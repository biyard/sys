use dioxus::prelude::*;
#[component]
pub fn Phone(
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
                d: "M12.8432 18.1568C14.5559 19.0188 16.5723 19.6404 18.9375 19.9063C19.5115 19.9709 20 19.511 20 18.9334V16.7808C20 16.3219 19.6877 15.9219 19.2425 15.8106L16.5493 15.1373C16.2085 15.0521 15.848 15.152 15.5996 15.4004L12.8432 18.1568ZM12.8432 18.1568C9.72749 16.5888 7.61705 14.225 6.24117 11.7588M6.24117 11.7588C4.93032 9.40926 4.28622 6.96674 4.07481 5.03084C4.01343 4.46884 4.46855 4 5.03389 4H7.1802C7.65688 4 8.06729 4.33646 8.16078 4.80388L8.89504 8.47521C8.96061 8.80307 8.85799 9.14201 8.62157 9.37843L6.24117 11.7588Z",
                stroke: "black",
                stroke_width: "2",
            }
        }
    }
}
#[component]
pub fn PhoneOff(
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
                d: "M19 5L5 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.96074 12.6251C7.79794 12.392 7.64295 12.1563 7.4955 11.9188L9.32872 10.0855C9.80157 9.61269 10.0068 8.93481 9.87567 8.27909L9.1414 4.60777C8.95443 3.67292 8.1336 3 7.18024 3H5.03393C3.92394 3 2.94939 3.93647 3.08076 5.1394C3.30497 7.1924 3.98451 9.76638 5.36793 12.246C5.7096 12.8585 6.09474 13.466 6.52708 14.0588L7.96074 12.6251ZM9.22026 14.194L7.80505 15.6092C9.03531 16.9311 10.5485 18.1214 12.3936 19.0501C14.2177 19.9682 16.3494 20.6216 18.8258 20.9001C20.025 21.0349 21 20.0745 21 18.9334V16.7808C21 15.863 20.3755 15.0631 19.4851 14.8405L16.7919 14.1672C16.1103 13.9968 15.3893 14.1965 14.8926 14.6933L12.6603 16.9255C11.3046 16.1572 10.1679 15.2222 9.22026 14.194ZM19 18.907C17.3414 18.7165 15.8745 18.3435 14.578 17.8363L16.3068 16.1075L19 16.7808V18.907ZM7.91451 8.67132L6.50966 10.0762C5.68624 8.27348 5.24739 6.47912 5.07765 5H7.18024L7.91451 8.67132Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Mobile(
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
                d: "M11 17H13M9 21H15C16.6569 21 18 19.6569 18 18V6C18 4.34315 16.6569 3 15 3H9C7.34315 3 6 4.34315 6 6V18C6 19.6569 7.34315 21 9 21Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn EndPhone(
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
                d: "M7.3864 8.71732C5.75255 9.25717 4.07992 10.1304 2.41001 11.4317C1.88873 11.8379 1.866 12.6058 2.33331 13.0731L3.46807 14.2079C3.8393 14.5791 4.41555 14.65 4.86574 14.3799L6.83091 13.2008C7.17553 12.994 7.3864 12.6216 7.3864 12.2197L7.3864 8.71732ZM7.3864 8.71732C10.4652 7.70005 13.4062 7.86658 15.9317 8.58325M15.9317 8.58325C18.2407 9.23848 20.2024 10.3536 21.6044 11.4443C22.1149 11.8415 22.126 12.5893 21.6686 13.0466L20.539 14.1763C20.1533 14.5619 19.5491 14.6218 19.0953 14.3192L16.4412 12.5498C16.1229 12.3376 15.9317 11.9804 15.9317 11.5979V8.58325Z",
                stroke: "black",
                stroke_width: "2",
            }
        }
    }
}
#[component]
pub fn MobileOff(
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
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.38062 4.29483C5.1365 4.81208 5 5.3901 5 6V18C5 20.2091 6.79086 22 9 22H15C17.2091 22 19 20.2091 19 18V17.9142L17 15.9142V18C17 19.1046 16.1046 20 15 20H9C7.89543 20 7 19.1046 7 18V6C7 5.97184 7.00058 5.94382 7.00173 5.91595L5.38062 4.29483ZM17 13.0858V6C17 4.89543 16.1046 4 15 4H9C8.68362 4 8.3844 4.07346 8.11847 4.20426L6.66565 2.75143C7.32258 2.27854 8.12876 2 9 2H15C17.2091 2 19 3.79086 19 6V15.0858L17 13.0858ZM11 16C10.4477 16 10 16.4477 10 17C10 17.5523 10.4477 18 11 18H13C13.5523 18 14 17.5523 14 17C14 16.4477 13.5523 16 13 16H11Z",
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
pub fn IncomingCall(
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
                d: "M12.8432 18.1568C14.5559 19.0188 16.5723 19.6404 18.9375 19.9063C19.5115 19.9709 20 19.511 20 18.9334V16.7808C20 16.3219 19.6877 15.9219 19.2425 15.8106L16.5493 15.1373C16.2085 15.0521 15.848 15.152 15.5996 15.4004L12.8432 18.1568ZM12.8432 18.1568C9.72749 16.5888 7.61705 14.225 6.24117 11.7588M6.24117 11.7588C4.93032 9.40926 4.28622 6.96674 4.07481 5.03084C4.01343 4.46884 4.46855 4 5.03389 4H7.1802C7.65688 4 8.06729 4.33646 8.16078 4.80388L8.89504 8.47521C8.96061 8.80307 8.85799 9.14201 8.62157 9.37843L6.24117 11.7588Z",
                stroke: "black",
                stroke_width: "2",
            }
            path {
                d: "M14 10H18M14 10V6M14 10L20 4",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn PhoneCall(
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
                d: "M12.8432 18.1568C14.5559 19.0188 16.5723 19.6404 18.9375 19.9063C19.5115 19.9709 20 19.511 20 18.9334V16.7808C20 16.3219 19.6877 15.9219 19.2425 15.8106L16.5493 15.1373C16.2085 15.0521 15.848 15.152 15.5996 15.4004L12.8432 18.1568ZM12.8432 18.1568C9.72749 16.5888 7.61705 14.225 6.24117 11.7588M6.24117 11.7588C4.93032 9.40926 4.28622 6.96674 4.07481 5.03084C4.01343 4.46884 4.46855 4 5.03389 4H7.1802C7.65688 4 8.06729 4.33646 8.16078 4.80388L8.89504 8.47521C8.96061 8.80307 8.85799 9.14201 8.62157 9.37843L6.24117 11.7588Z",
                stroke: "black",
                stroke_width: "2",
            }
            path {
                d: "M12.9541 7.0919C13.938 7.28315 14.8417 7.76567 15.5479 8.47687C16.2542 9.18806 16.7304 10.0951 16.9147 11.0803M13.0969 3.06708C15.0793 3.31049 16.9245 4.20646 18.3418 5.61386C19.759 7.02126 20.6679 8.86012 20.9251 10.8408",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn MissedCall(
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
                d: "M7.3864 13.7173C5.75255 14.2572 4.07992 15.1304 2.41001 16.4317C1.88873 16.8379 1.866 17.6058 2.33331 18.0731L3.46807 19.2079C3.8393 19.5791 4.41555 19.65 4.86574 19.3799L6.83091 18.2008C7.17553 17.994 7.3864 17.6216 7.3864 17.2197L7.3864 13.7173ZM7.3864 13.7173C10.4652 12.7001 13.4062 12.8666 15.9317 13.5832M15.9317 13.5832C18.2407 14.2385 20.2024 15.3536 21.6044 16.4443C22.1149 16.8415 22.126 17.5893 21.6686 18.0466L20.539 19.1763C20.1533 19.5619 19.5491 19.6218 19.0953 19.3192L16.4412 17.5498C16.1229 17.3376 15.9317 16.9804 15.9317 16.5979V13.5832Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linejoin: "round",
            }
            path {
                d: "M16.5 4.5L12.7071 8.29289C12.3166 8.68342 11.6834 8.68342 11.2929 8.29289L7 4M7 4V7M7 4H10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ForwadedCall(
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
                d: "M12.8432 18.1568C14.5559 19.0188 16.5723 19.6404 18.9375 19.9063C19.5115 19.9709 20 19.511 20 18.9334V16.7808C20 16.3219 19.6877 15.9219 19.2425 15.8106L16.5493 15.1373C16.2085 15.0521 15.848 15.152 15.5996 15.4004L12.8432 18.1568ZM12.8432 18.1568C9.72749 16.5888 7.61705 14.225 6.24117 11.7588M6.24117 11.7588C4.93032 9.40926 4.28622 6.96674 4.07481 5.03084C4.01343 4.46884 4.46855 4 5.03389 4H7.1802C7.65688 4 8.06729 4.33646 8.16078 4.80388L8.89504 8.47521C8.96061 8.80307 8.85799 9.14201 8.62157 9.37843L6.24117 11.7588Z",
                stroke: "black",
                stroke_width: "2",
            }
            path {
                d: "M20 4H16M20 4V8M20 4L14 10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
