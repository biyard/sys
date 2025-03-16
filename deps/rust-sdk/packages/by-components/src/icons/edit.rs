use dioxus::prelude::*;
#[component]
pub fn Scissors(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "".to_string())] class: String,
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
                cx: "6.5",
                cy: "6.5",
                r: "2.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6.5",
                cy: "17.5",
                r: "2.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 15L20 6.26764",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M12.6984 11.9995L6.52923 8.15153C6.06063 7.85925 5.44381 8.00218 5.15152 8.47078C4.85924 8.93939 5.00217 9.55621 5.47077 9.84849L10.8089 13.1781L12.6984 11.9995ZM14.5879 13.1781L12.6984 14.3566L19.4708 18.5809C19.9394 18.8731 20.5562 18.7302 20.8485 18.2616C21.1408 17.793 20.9979 17.1762 20.5293 16.8839L14.5879 13.1781Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Delete1(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M4 7H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 10L7.70141 19.3578C7.87432 20.3088 8.70258 21 9.66915 21H14.3308C15.2974 21 16.1257 20.3087 16.2986 19.3578L18 10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FlipVertical(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M10 19V5H8L3 19H10Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 19V5H16L21 19H14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn PushPin(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M8.84699 5H8C7.44772 5 7 4.55228 7 4C7 3.44772 7.44772 3 8 3H10H14H16C16.5523 3 17 3.44772 17 4C17 4.55228 16.5523 5 16 5H15.153L15.8666 9.99517C16.2261 10.1989 16.5668 10.4329 16.8828 10.6962C18.2091 11.8015 19 13.3437 19 15C19 15.5523 18.5523 16 18 16H12L6 16C5.44772 16 5 15.5523 5 15C5 13.3437 5.79093 11.8015 7.11718 10.6962C7.43316 10.4329 7.77385 10.1989 8.13339 9.99517L8.84699 5ZM10.8673 5L10.0406 10.7872C9.99294 11.1205 9.78085 11.4074 9.47623 11.5507C9.07954 11.7373 8.71669 11.9667 8.39754 12.2327C7.78492 12.7432 7.3704 13.3553 7.16053 14L12 14H16.8395C16.6296 13.3553 16.2151 12.7432 15.6025 12.2327C15.2833 11.9667 14.9205 11.7373 14.5238 11.5507C14.2191 11.4074 14.0071 11.1205 13.9594 10.7872L13.1327 5H10.8673Z",
                fill: "black",
            }
            path {
                d: "M12 15V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FlipHorizontal(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
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
                d: "M5 14L19 14L19 16L5 21L5 14Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5 10L19 10L19 8L5 3L5 10Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Search(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "black".to_string())] color: String,
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
                cx: "10",
                cy: "10",
                r: "6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14.5 14.5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Edit1(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
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
                d: "M12 3.99997H6C4.89543 3.99997 4 4.8954 4 5.99997V18C4 19.1045 4.89543 20 6 20H18C19.1046 20 20 19.1045 20 18V12M18.4142 8.41417L19.5 7.32842C20.281 6.54737 20.281 5.28104 19.5 4.5C18.7189 3.71895 17.4526 3.71895 16.6715 4.50001L15.5858 5.58575M18.4142 8.41417L12.3779 14.4505C12.0987 14.7297 11.7431 14.9201 11.356 14.9975L8.41422 15.5858L9.00257 12.6441C9.08001 12.2569 9.27032 11.9013 9.54951 11.6221L15.5858 5.58575M18.4142 8.41417L15.5858 5.58575",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FlipHorizontal2(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M19 12L17 12M13 12L11 12M7 12L5 12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17 20L12 15L7 20L17 20Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17 4L12 9L7 4L17 4Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Eye(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
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
                d: "M21.3351 11.4069L22.2682 11.0474L21.3351 11.4069ZM21.3351 12.5932L20.4019 12.2337L21.3351 12.5932ZM2.66495 11.4068L1.73178 11.0474L2.66495 11.4068ZM2.66495 12.5932L1.73178 12.9526L2.66495 12.5932ZM3.59813 11.7663C4.89787 8.39171 8.17087 6 12 6V4C7.31644 4 3.31892 6.92667 1.73178 11.0474L3.59813 11.7663ZM12 6C15.8291 6 19.1021 8.39172 20.4019 11.7663L22.2682 11.0474C20.6811 6.92668 16.6836 4 12 4V6ZM20.4019 12.2337C19.1021 15.6083 15.8291 18 12 18V20C16.6836 20 20.6811 17.0733 22.2682 12.9526L20.4019 12.2337ZM12 18C8.17087 18 4.89787 15.6083 3.59813 12.2337L1.73178 12.9526C3.31892 17.0733 7.31644 20 12 20V18ZM20.4019 11.7663C20.4598 11.9165 20.4598 12.0835 20.4019 12.2337L22.2682 12.9526C22.5043 12.3396 22.5043 11.6604 22.2682 11.0474L20.4019 11.7663ZM1.73178 11.0474C1.4957 11.6604 1.4957 12.3396 1.73178 12.9526L3.59813 12.2337C3.54025 12.0835 3.54025 11.9165 3.59813 11.7663L1.73178 11.0474Z",
                fill: "black",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "3",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Brush(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M8 13.5L10.7817 10.7183M10 16.5L13.2817 13.2183M10.7817 10.7183L17.2506 5.2123C17.4158 5.0717 17.6027 4.95892 17.8041 4.87836L17.8336 4.86658C18.6499 4.54005 19.4599 5.35013 19.1334 6.16645L19.1216 6.19589C19.0411 6.39729 18.9283 6.58424 18.7877 6.74942L13.2817 13.2183M10.7817 10.7183C11.615 10.7911 13.2817 11.393 13.2817 13.2183",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4.4853 17.2881C4.16795 15.5783 5.48027 14 7.2193 14V14C8.75504 14 10 15.245 10 16.7807V17C10 18.6569 8.65685 20 7 20H3.92627C3.6329 20 3.51004 19.6253 3.74646 19.4516L4.55841 18.855C4.67556 18.769 4.73314 18.6235 4.70662 18.4805L4.4853 17.2881Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn MoveUp(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
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
                x: "14",
                y: "4",
                width: "6",
                height: "6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "14",
                y: "14",
                width: "6",
                height: "6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 19.3733C8.4087 19.3733 6.88258 18.7411 5.75736 17.6159C4.63214 16.4907 4 14.9646 4 13.3733C4 11.782 4.63214 10.2559 5.75736 9.13065C6.88258 8.00543 8.4087 7.37329 10 7.37329",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8.39581 5.54474L10.9255 6.98488L9.48533 9.51453",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Delete(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
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
                d: "M10 11V17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 11V17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 7H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 7H12H18V18C18 19.6569 16.6569 21 15 21H9C7.34315 21 6 19.6569 6 18V7Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FlipHorizontal1(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M3 13H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M8.2 4H8.16146C7.63431 3.99998 7.17955 3.99997 6.80497 4.03057C6.40963 4.06287 6.01641 4.13419 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4.13419 6.01641 4.06287 6.40963 4.03057 6.80497C3.99997 7.17954 3.99998 7.63431 4 8.16146V8.2V10H6V8.2C6 7.62345 6.00078 7.25117 6.02393 6.96784C6.04612 6.69617 6.0838 6.59545 6.10899 6.54601C6.20487 6.35785 6.35785 6.20487 6.54601 6.10899C6.59545 6.0838 6.69617 6.04612 6.96784 6.02393C7.25117 6.00078 7.62345 6 8.2 6H15.8C16.3766 6 16.7488 6.00078 17.0322 6.02393C17.3038 6.04612 17.4045 6.0838 17.454 6.10899C17.6422 6.20487 17.7951 6.35785 17.891 6.54601C17.9162 6.59545 17.9539 6.69617 17.9761 6.96784C17.9992 7.25117 18 7.62345 18 8.2V10H20V8.2V8.16148C20 7.63432 20 7.17955 19.9694 6.80497C19.9371 6.40963 19.8658 6.01641 19.673 5.63803C19.3854 5.07354 18.9265 4.6146 18.362 4.32698C17.9836 4.13419 17.5904 4.06287 17.195 4.03057C16.8205 3.99997 16.3657 3.99998 15.8385 4H15.8H8.2ZM20 12H18V15.8C18 16.3766 17.9992 16.7488 17.9761 17.0322C17.9539 17.3038 17.9162 17.4045 17.891 17.454C17.7951 17.6422 17.6422 17.7951 17.454 17.891C17.4045 17.9162 17.3038 17.9539 17.0322 17.9761C16.7488 17.9992 16.3766 18 15.8 18H8.2C7.62345 18 7.25117 17.9992 6.96784 17.9761C6.69617 17.9539 6.59545 17.9162 6.54601 17.891C6.35785 17.7951 6.20487 17.6422 6.10899 17.454C6.0838 17.4045 6.04612 17.3038 6.02393 17.0322C6.00078 16.7488 6 16.3766 6 15.8V12H4V15.8V15.8385C3.99998 16.3657 3.99997 16.8205 4.03057 17.195C4.06287 17.5904 4.13419 17.9836 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.01641 19.8658 6.40963 19.9371 6.80497 19.9694C7.17955 20 7.63432 20 8.16148 20H8.2H15.8H15.8385C16.3657 20 16.8205 20 17.195 19.9694C17.5904 19.9371 17.9836 19.8658 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C19.8658 17.9836 19.9371 17.5904 19.9694 17.195C20 16.8205 20 16.3657 20 15.8385V15.8V12Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn EditContent(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M4 5L15 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 8H15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 11H11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.4563 13.5422L13.9268 18.0718C13.6476 18.351 13.292 18.5413 12.9048 18.6188L10.8153 19.0367L11.2332 16.9472C11.3106 16.56 11.5009 16.2044 11.7801 15.9252L16.3096 11.3956M18.4563 13.5422L19.585 12.4135C19.9755 12.0229 19.9755 11.3898 19.585 10.9993L18.8526 10.2669C18.4621 9.87634 17.8289 9.87634 17.4384 10.2669L16.3096 11.3956M18.4563 13.5422L16.3096 11.3956",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Delete3(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
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
                d: "M4 7H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 7V18C6 19.6569 7.34315 21 9 21H15C16.6569 21 18 19.6569 18 18V7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn PickerOff(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
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
                d: "M19.0857 3.49997C18.3047 2.71892 17.0383 2.71892 16.2573 3.49997L14.1716 5.58569L12.8787 4.29287C12.4882 3.90234 11.8551 3.90234 11.4645 4.29287C11.074 4.68339 11.074 5.31656 11.4645 5.70708L12.7574 6.99991L11.5858 8.17153L13 9.58574L14.1716 8.41413L15.5858 9.82834L14.4142 11L15.8284 12.4142L17 11.2426L18.2929 12.5354C18.6834 12.926 19.3166 12.926 19.7071 12.5354C20.0976 12.1449 20.0976 11.5118 19.7071 11.1212L18.4142 9.82833L20.4999 7.74261C21.281 6.96156 21.281 5.69523 20.4999 4.91418L19.0857 3.49997ZM14.4142 13.8284L13 12.4142L8.25661 17.1576C8.18182 17.2324 8.09607 17.2944 8.0028 17.3419L6.64189 16.0307C6.69105 15.9244 6.75874 15.8271 6.8424 15.7434L11.5858 11L10.1716 9.58575L5.42817 14.3292C5.00938 14.748 4.72392 15.2814 4.60777 15.8622L4.54975 16.1523L4.37297 17.0361L4.01942 18.8039C3.95384 19.1318 4.05647 19.4707 4.29289 19.7071C4.52931 19.9436 4.86825 20.0462 5.19611 19.9806L7.9144 19.437L8.13785 19.3923C8.71863 19.2761 9.25203 18.9906 9.67083 18.5718L14.4142 13.8284ZM15.5858 6.99991L17 8.41412L19.0857 6.3284L17.6715 4.91418L15.5858 6.99991Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn FlipVertical1(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
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
                d: "M11 3L11 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M20 8.2L20 8.16146C20 7.63431 20 7.17955 19.9694 6.80497C19.9371 6.40963 19.8658 6.01641 19.673 5.63803C19.3854 5.07354 18.9265 4.6146 18.362 4.32698C17.9836 4.13419 17.5904 4.06287 17.195 4.03057C16.8205 3.99997 16.3657 3.99998 15.8385 4L15.8 4L14 4L14 6L15.8 6C16.3766 6 16.7488 6.00078 17.0322 6.02393C17.3038 6.04612 17.4045 6.0838 17.454 6.10899C17.6422 6.20487 17.7951 6.35785 17.891 6.54601C17.9162 6.59545 17.9539 6.69617 17.9761 6.96784C17.9992 7.25117 18 7.62345 18 8.2L18 15.8C18 16.3766 17.9992 16.7488 17.9761 17.0322C17.9539 17.3038 17.9162 17.4045 17.891 17.454C17.7951 17.6422 17.6422 17.7951 17.454 17.891C17.4045 17.9162 17.3038 17.9539 17.0322 17.9761C16.7488 17.9992 16.3766 18 15.8 18L14 18L14 20L15.8 20L15.8385 20C16.3657 20 16.8204 20 17.195 19.9694C17.5904 19.9371 17.9836 19.8658 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C19.8658 17.9836 19.9371 17.5904 19.9694 17.195C20 16.8205 20 16.3657 20 15.8385L20 15.8L20 8.2ZM12 20L12 18L8.2 18C7.62344 18 7.25117 17.9992 6.96783 17.9761C6.69617 17.9539 6.59545 17.9162 6.54601 17.891C6.35785 17.7951 6.20486 17.6422 6.10899 17.454C6.0838 17.4045 6.04612 17.3038 6.02393 17.0322C6.00078 16.7488 6 16.3766 6 15.8L6 8.2C6 7.62345 6.00078 7.25117 6.02393 6.96784C6.04612 6.69617 6.0838 6.59545 6.10899 6.54601C6.20487 6.35785 6.35785 6.20487 6.54601 6.10899C6.59545 6.0838 6.69617 6.04612 6.96783 6.02393C7.25117 6.00078 7.62345 6 8.2 6L12 6L12 4L8.2 4L8.16146 4C7.63431 3.99998 7.17954 3.99997 6.80497 4.03057C6.40963 4.06287 6.01641 4.13419 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4.13419 6.01641 4.06287 6.40963 4.03057 6.80497C3.99997 7.17955 3.99998 7.63432 4 8.16148L4 8.2L4 15.8L4 15.8385C3.99998 16.3657 3.99997 16.8205 4.03057 17.195C4.06287 17.5904 4.13419 17.9836 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.01641 19.8658 6.40963 19.9371 6.80497 19.9694C7.17955 20 7.63432 20 8.16148 20L8.2 20L12 20Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn ZoomIn(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
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
                cx: "10",
                cy: "10",
                r: "6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14.5 14.5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 10H10M12 10H10M10 10V8M10 10V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Edit2(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M18.3786 8.44975L11.4638 15.3647C11.1846 15.6439 10.829 15.8342 10.4418 15.9117L7.50006 16.5L8.08841 13.5582C8.16584 13.1711 8.35615 12.8155 8.63534 12.5363L15.5502 5.62132M18.3786 8.44975L19.7928 7.03553C20.1834 6.64501 20.1834 6.01184 19.7928 5.62132L18.3786 4.20711C17.9881 3.81658 17.3549 3.81658 16.9644 4.20711L15.5502 5.62132M18.3786 8.44975L15.5502 5.62132",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5 20H19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Crop(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
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
                d: "M20 16L10 16C8.89543 16 8 15.1046 8 14L8 4",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M17 13V10C17 8.34315 15.6569 7 14 7H11V9H14C14.5523 9 15 9.44772 15 10V13H17ZM9 9V7H4C3.44772 7 3 7.44772 3 8C3 8.55228 3.44772 9 4 9H9ZM15 15H17V20C17 20.5523 16.5523 21 16 21C15.4477 21 15 20.5523 15 20V15Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn EyeOff(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M6.22311 5.63732C4.19215 6.89322 2.60072 8.79137 1.73178 11.0474C1.4957 11.6604 1.4957 12.3396 1.73178 12.9526C3.31892 17.0733 7.31644 20 12 20C14.4221 20 16.6607 19.2173 18.4773 17.8915L17.042 16.4562C15.6033 17.4309 13.8678 18 12 18C8.17087 18 4.89787 15.6083 3.59813 12.2337C3.54025 12.0835 3.54025 11.9165 3.59813 11.7663C4.36734 9.76914 5.82769 8.11625 7.68543 7.09964L6.22311 5.63732ZM9.47958 8.89379C8.57683 9.6272 8 10.7462 8 12C8 14.2091 9.79086 16 12 16C13.2538 16 14.3728 15.4232 15.1062 14.5204L13.6766 13.0908C13.3198 13.6382 12.7021 14 12 14C10.8954 14 10 13.1046 10 12C10 11.2979 10.3618 10.6802 10.9092 10.3234L9.47958 8.89379ZM15.9627 12.5485L11.4515 8.03729C11.6308 8.0127 11.8139 8 12 8C14.2091 8 16 9.79086 16 12C16 12.1861 15.9873 12.3692 15.9627 12.5485ZM18.5678 15.1536C19.3539 14.3151 19.9812 13.3259 20.4019 12.2337C20.4598 12.0835 20.4598 11.9165 20.4019 11.7663C19.1021 8.39172 15.8291 6 12 6C11.2082 6 10.4402 6.10226 9.70854 6.29433L8.11858 4.70437C9.32544 4.24913 10.6335 4 12 4C16.6836 4 20.6811 6.92668 22.2682 11.0474C22.5043 11.6604 22.5043 12.3396 22.2682 12.9526C21.7464 14.3074 20.964 15.5331 19.9824 16.5682L18.5678 15.1536Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn MoveDown(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
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
                x: "14",
                y: "4",
                width: "6",
                height: "6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "14",
                y: "14",
                width: "6",
                height: "6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 5C8.4087 5 6.88258 5.63214 5.75736 6.75736C4.63214 7.88258 4 9.4087 4 11C4 12.5913 4.63214 14.1174 5.75736 15.2426C6.88258 16.3679 8.4087 17 10 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8.39581 18.8286L10.9255 17.3884L9.48533 14.8588",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Picker(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M17 9.82842L8.96373 17.8648C8.68452 18.144 8.32892 18.3343 7.94174 18.4118L7.71828 18.4565M17 9.82842L19.7928 7.03559C20.1834 6.64507 20.1834 6.0119 19.7928 5.62138L18.3786 4.20716C17.9881 3.81664 17.3549 3.81664 16.9644 4.20716L14.1716 6.99999M17 9.82842L14.1716 6.99999M17 9.82842L19 11.8284M14.1716 6.99999L6.13529 15.0364C5.85609 15.3156 5.66579 15.6712 5.58835 16.0584L5.53033 16.3485M14.1716 6.99999L12.1716 5.00006M5.53033 16.3485L5.35355 17.2323L5 19.0001L7.71828 18.4565M5.53033 16.3485L7.71828 18.4565",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FlipVertical2(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M12 5V7M12 11V13M12 17V19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M20 7L15 12L20 17V7Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 7L9 12L4 17V7Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Delete2(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M10 12V17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 12V17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 7H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 10V18C6 19.6569 7.34315 21 9 21H15C16.6569 21 18 19.6569 18 18V10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Brush1(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
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
                d: "M4 17C4 15.3431 5.34315 14 7 14V14C8.65685 14 10 15.3431 10 17V17C10 18.6569 8.65685 20 7 20H4.54545C4.24421 20 4 19.7558 4 19.4545V18.5V17V17Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 13.7542L15.5898 5.32104C16.3563 4.46932 17.6804 4.4345 18.4906 5.24475L18.6229 5.37708L18.7552 5.5094C19.5655 6.31965 19.5307 7.64365 18.679 8.4102L10.2458 16",
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
    #[props(default = "24".to_string())] height: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
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
                cx: "10",
                cy: "10",
                r: "6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14.5 14.5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 10H10H12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Edit(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "".to_string())] class: String,
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
                d: "M18.3786 8.44975L8.96375 17.8648C8.68455 18.144 8.32895 18.3343 7.94176 18.4117L5.00003 19.0001L5.58838 16.0583C5.66581 15.6711 5.85612 15.3155 6.13532 15.0363L15.5502 5.62132M18.3786 8.44975L19.7929 7.03553C20.1834 6.64501 20.1834 6.01184 19.7929 5.62132L18.3786 4.20711C17.9881 3.81658 17.355 3.81658 16.9644 4.20711L15.5502 5.62132M18.3786 8.44975L15.5502 5.62132",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
