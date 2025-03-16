use dioxus::prelude::*;
#[component]
pub fn Launch2(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M10.1429 7H7C5.89543 7 5 7.89543 5 9V17C5 18.1046 5.89543 19 7 19H15C16.1046 19 17 18.1046 17 17V13.2143M19 5H15M19 5V9M19 5L9 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Wifi(
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
            circle {
                cx: "1",
                cy: "1",
                r: "1",
                transform: "matrix(1 0 0 -1 11 19)",
                fill: "black",
            }
            path {
                d: "M20.1915 10.2642C19.2864 8.97153 18.0881 7.9114 16.6947 7.17053C15.3013 6.42965 13.7524 6.02907 12.1745 6.00152C10.5967 5.97398 9.03468 6.32026 7.61629 7.01206C6.19789 7.70385 4.96337 8.72152 4.01364 9.98185M17.7341 12.985C17.1004 12.0801 16.2617 11.338 15.2863 10.8194C14.3109 10.3008 13.2267 10.0203 12.1222 10.0011C11.0177 9.98179 9.92427 10.2242 8.9314 10.7084C7.93853 11.1927 7.07436 11.9051 6.40955 12.7873M14.4575 15.2793C14.1859 14.8915 13.8264 14.5734 13.4084 14.3512C12.9904 14.1289 12.5257 14.0087 12.0524 14.0005C11.579 13.9922 11.1104 14.0961 10.6849 14.3036C10.2594 14.5112 9.88901 14.8165 9.60409 15.1946",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LinkBroke(
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
                d: "M8.37032 11.0726L5.41421 14.0287C4.63317 14.8097 4.63316 16.076 5.41421 16.8571L6.95611 18.399C7.73715 19.18 9.00348 19.18 9.78453 18.399L12.7406 15.4429M11.0726 8.37032L14.0287 5.41421C14.8097 4.63317 16.076 4.63316 16.8571 5.41421L18.399 6.95611C19.18 7.73715 19.18 9.00348 18.399 9.78453L15.4429 12.7406M6.64883 6.64883L4.88296 4.88296M19.0992 19.0992L17.3333 17.3333M9.35119 5.87299V4M14.6488 20V18.127M5.87299 9.35119H4M20 14.6488H18.127",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Signal(
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
                d: "M7 14V18M12 10V18M17 6V18",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Link1(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M10 8H6C4.34315 8 3 9.34315 3 11V13C3 14.6569 4.34315 16 6 16H10M9 12H15M14 8H18C19.6569 8 21 9.34315 21 11V13C21 14.6569 19.6569 16 18 16H14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Send1(
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
                d: "M20 12L4 4L6 12M20 12L4 20L6 12M20 12H6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Satellite(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
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
                d: "M14.4749 11.4247C14.6142 11.2854 14.7247 11.12 14.8 10.938C14.8754 10.756 14.9142 10.561 14.9142 10.364C14.9142 10.167 14.8754 9.97196 14.8 9.78997C14.7247 9.60798 14.6142 9.44262 14.4749 9.30334C14.3356 9.16405 14.1702 9.05356 13.9882 8.97818C13.8063 8.90279 13.6112 8.864 13.4142 8.864C13.2172 8.864 13.0222 8.90279 12.8402 8.97818C12.6582 9.05356 12.4928 9.16405 12.3536 9.30334L13.4142 10.364L14.4749 11.4247Z",
                fill: "black",
            }
            path {
                d: "M8.05887 15.9411C7.40613 15.2883 6.88835 14.5134 6.53508 13.6606C6.18182 12.8077 6 11.8936 6 10.9705C6 10.0474 6.18182 9.13331 6.53508 8.28046C6.88835 7.42761 7.40613 6.65269 8.05888 5.99995L18 15.9411C17.3473 16.5938 16.5723 17.1116 15.7195 17.4649C14.8666 17.8181 13.9526 17.9999 13.0294 17.9999C12.1063 17.9999 11.1922 17.8181 10.3394 17.4649C9.48654 17.1116 8.71162 16.5938 8.05887 15.9411ZM8.05887 15.9411L5 21H10M17.6025 9.0463C17.1056 7.87566 16.1813 6.93835 15.0177 6.42515M20.6901 8.79485C20.3187 7.49954 19.6261 6.31904 18.6766 5.36288C17.7271 4.40672 16.5514 3.7059 15.2587 3.32544",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Share3(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M15.2141 7.39294L8.68387 10.6581M8.68387 10.6581C8.19134 9.67492 7.17449 9 6 9C4.34315 9 3 10.3431 3 12C3 13.6569 4.34315 15 6 15C7.17449 15 8.19134 14.3251 8.68387 13.3419M8.68387 10.6581C8.88616 11.0619 9 11.5176 9 12C9 12.4824 8.88616 12.9381 8.68387 13.3419M15.2141 16.6071L8.68387 13.3419M21 6C21 7.65685 19.6569 9 18 9C16.3431 9 15 7.65685 15 6C15 4.34315 16.3431 3 18 3C19.6569 3 21 4.34315 21 6ZM21 18C21 19.6569 19.6569 21 18 21C16.3431 21 15 19.6569 15 18C15 16.3431 16.3431 15 18 15C19.6569 15 21 16.3431 21 18Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn WifiOff(
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
                d: "M5 5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.19213 6.10634C7.18739 6.10865 7.18265 6.11095 7.17791 6.11327C5.61768 6.87424 4.2597 7.99367 3.215 9.38003C2.88263 9.82111 2.97075 10.4481 3.41182 10.7805C3.8529 11.1129 4.4799 11.0247 4.81227 10.5837C5.66703 9.44937 6.7781 8.53347 8.05465 7.91085C8.26931 7.80616 8.48763 7.71025 8.70905 7.62326L7.19213 6.10634ZM10.2742 9.18837C9.66015 9.32403 9.06214 9.53207 8.49302 9.80965C7.35831 10.3631 6.37069 11.1772 5.61091 12.1855C5.27854 12.6266 5.36665 13.2536 5.80773 13.5859C6.2488 13.9183 6.87581 13.8302 7.20818 13.3891C7.77802 12.6329 8.51873 12.0223 9.36977 11.6072C10.2153 11.1948 11.1459 10.9871 12.0864 11.0006L10.2742 9.18837ZM17.8876 13.9733L12.9737 9.05948C13.9446 9.17853 14.8879 9.47497 15.7558 9.93642C16.8705 10.5291 17.8291 11.3772 18.5532 12.4114C18.87 12.8638 18.76 13.4873 18.3076 13.8041C18.1779 13.8949 18.0341 13.9507 17.8876 13.9733ZM10.973 7.05879L9.26074 5.34653C10.2147 5.10125 11.2005 4.98437 12.192 5.00168C13.9276 5.03197 15.6315 5.47261 17.1642 6.28758C18.6969 7.10254 20.015 8.26868 21.0107 9.69066C21.3274 10.1431 21.2175 10.7666 20.7651 11.0834C20.3127 11.4002 19.6891 11.2902 19.3724 10.8378C18.5577 9.67438 17.4793 8.72026 16.2252 8.05347C14.9712 7.38669 13.5771 7.02616 12.1571 7.00137C11.7604 6.99445 11.3649 7.01377 10.973 7.05879ZM13 18C13 17.4477 12.5523 17 12 17C11.4477 17 11 17.4477 11 18C11 18.5523 11.4477 19 12 19C12.5523 19 13 18.5523 13 18ZM10.2465 13.4048C10.8139 13.1281 11.4387 12.9896 12.0698 13.0006C12.7009 13.0116 13.3205 13.1719 13.8779 13.4682C14.4352 13.7646 14.9145 14.1886 15.2766 14.7057C15.5934 15.1581 15.4834 15.7816 15.031 16.0984C14.5786 16.4152 13.9551 16.3053 13.6383 15.8528C13.4573 15.5943 13.2176 15.3823 12.9389 15.2341C12.6603 15.0859 12.3505 15.0058 12.0349 15.0003C11.7193 14.9948 11.4069 15.0641 11.1233 15.2024C10.8396 15.3408 10.5927 15.5443 10.4027 15.7964C10.0703 16.2374 9.44335 16.3256 9.00227 15.9932C8.5612 15.6608 8.47308 15.0338 8.80545 14.5927C9.18534 14.0886 9.67915 13.6815 10.2465 13.4048Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Link2(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
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
            path {
                d: "M7.75737 10.5858L4.92894 13.4142C3.75737 14.5858 3.75737 16.4853 4.92894 17.6569L6.34316 19.0711C7.51473 20.2427 9.41422 20.2427 10.5858 19.0711L13.4142 16.2427M9.8787 14.1214L14.1213 9.87873M10.5858 7.7574L13.4142 4.92897C14.5858 3.7574 16.4853 3.7574 17.6568 4.92897L19.0711 6.34319C20.2426 7.51476 20.2426 9.41425 19.0711 10.5858L16.2426 13.4143",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LinkOff2(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M7.05026 11.2929L4.92894 13.4142C3.75737 14.5858 3.75737 16.4853 4.92894 17.6568L6.34316 19.0711C7.51473 20.2426 9.41422 20.2426 10.5858 19.0711L12.7071 16.9497M5 5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M11.8358 10.75L9.17156 13.4142C8.78103 13.8048 8.78103 14.4379 9.17156 14.8284C9.56208 15.219 10.1952 15.219 10.5858 14.8284L13.25 12.1642L11.8358 10.75ZM17.4926 13.5785L16.0784 12.1643L16.2426 12L17.6568 10.5858L18.3639 9.87872C19.145 9.09767 19.145 7.83134 18.3639 7.05029L16.9497 5.63608C16.1687 4.85503 14.9024 4.85503 14.1213 5.63608L13.4142 6.34319L12 7.7574L11.8358 7.92161L10.4216 6.5074L10.5858 6.34319L12 4.92897L12.7071 4.22187C14.2692 2.65977 16.8019 2.65977 18.3639 4.22187L19.7782 5.63608C21.3403 7.19818 21.3403 9.73084 19.7782 11.2929L19.0711 12L17.6568 13.4143L17.4926 13.5785ZM13.25 9.33581L13.4142 9.1716C13.8047 8.78107 14.4379 8.78107 14.8284 9.1716C15.2189 9.56212 15.2189 10.1953 14.8284 10.5858L14.6642 10.75L13.25 9.33581Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Send2(
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
                d: "M20 4L3 9.31372L10.5 13.5M20 4L14.5 21L10.5 13.5M20 4L10.5 13.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Share2(
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
            path {
                d: "M8 11H6C4.89543 11 4 11.8954 4 13V19C4 20.1046 4.89543 21 6 21H18C19.1046 21 20 20.1046 20 19V13C20 11.8954 19.1046 11 18 11H16M12 17V3M12 3L9 6M12 3L15 6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Share1(
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
                d: "M5 12V17C5 18.6569 6.34315 20 8 20H16C17.6569 20 19 18.6569 19 17V12M12 16V4M12 4L8 8M12 4L16 8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Launch1(
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
                d: "M11 5H7C5.89543 5 5 5.89543 5 7V17C5 18.1046 5.89543 19 7 19H17C18.1046 19 19 18.1046 19 17V12.25M19 5H15M19 5V9M19 5L9 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LinkOff1(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
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
                d: "M5 5L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.58579 7H7H6C3.79086 7 2 8.79086 2 11V13C2 15.2091 3.79086 17 6 17H7H9H10C10.5523 17 11 16.5523 11 16C11 15.4477 10.5523 15 10 15H9H7H6C4.89543 15 4 14.1046 4 13V11C4 9.89543 4.89543 9 6 9H7H9H9.58579L7.58579 7ZM11.5858 11H9C8.44772 11 8 11.4477 8 12C8 12.5523 8.44772 13 9 13H13.5858L11.5858 11ZM15.5858 15H15H14C13.4477 15 13 15.4477 13 16C13 16.5523 13.4477 17 14 17H15H17H17.5858L15.5858 15ZM19.9227 16.5085L18.3785 14.9642C19.3021 14.7873 20 13.9752 20 13V11C20 9.89543 19.1046 9 18 9H17H15H14C13.4477 9 13 8.55228 13 8C13 7.44772 13.4477 7 14 7H15H17H18C20.2091 7 22 8.79086 22 11V13C22 14.5123 21.1608 15.8285 19.9227 16.5085ZM15.8834 12.4691L14.4142 11H15C15.5523 11 16 11.4477 16 12C16 12.1695 15.9578 12.3292 15.8834 12.4691Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Link3(
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
                d: "M10.6667 13.3333L10.0809 12.7475C9.29984 11.9665 9.29984 10.7002 10.0809 9.91911L14.5858 5.41422C15.3668 4.63317 16.6332 4.63317 17.4142 5.41422L18.5858 6.58578C19.3668 7.36683 19.3668 8.63316 18.5858 9.41421L17 11M13.3333 10.6667L13.9191 11.2525C14.7002 12.0335 14.7002 13.2998 13.9191 14.0809L9.41421 18.5858C8.63316 19.3668 7.36683 19.3668 6.58579 18.5858L5.41422 17.4142C4.63317 16.6332 4.63317 15.3668 5.41422 14.5858L7 13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
