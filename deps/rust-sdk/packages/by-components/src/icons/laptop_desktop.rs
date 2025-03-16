use dioxus::prelude::*;
#[component]
pub fn OfflineLaptop(
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
                d: "M11 5H7C5.89543 5 5 5.89543 5 7V16H19V13.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.0251 5.02513C14.3918 5.6585 14 6.5335 14 7.5C14 9.433 15.567 11 17.5 11C18.4665 11 19.3415 10.6082 19.9749 9.97487M15.0251 5.02513C15.6585 4.39175 16.5335 4 17.5 4C19.433 4 21 5.567 21 7.5C21 8.4665 20.6082 9.3415 19.9749 9.97487M15.0251 5.02513L17.5 7.5L19.9749 9.97487",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn DownloadDesktop(
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
                d: "M12 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.5 3V9M17.5 9L15 6.5M17.5 9L20 6.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FavouriteLaptop(
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
                d: "M12 5H7C5.89543 5 5 5.89543 5 7V16H19V12.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.5147 4.51469C14.8284 5.20098 14.8284 6.31368 15.5147 6.99997L18 9.48535L20.4853 7.00007C21.1716 6.31378 21.1716 5.20108 20.4853 4.51479C19.799 3.82851 18.6863 3.8285 18 4.51474C17.3138 3.82845 16.201 3.8284 15.5147 4.51469Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AddDesktop(
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
                d: "M11 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18 3V6M18 9V6M18 6H15M18 6H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn RemoveDesktop(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M11 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M20.8787 3.87866L18.3787 6.37866M18.3787 6.37866L15.8787 8.87866M18.3787 6.37866L15.8787 3.87866M18.3787 6.37866L20.8787 8.87866",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn UploadLaptop(
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
                d: "M13 5H7C5.89543 5 5 5.89543 5 7V16H19V12.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.5 9L18.5 3M18.5 3L21 5.5M18.5 3L16 5.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Laptop(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M5 7C5 5.89543 5.89543 5 7 5H17C18.1046 5 19 5.89543 19 7V16H5V7Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn UploadDesktop(
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
                d: "M12 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.5 9V3M17.5 3L15 5.5M17.5 3L20 5.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn DownloadLaptop(
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
                d: "M12 5H7C5.89543 5 5 5.89543 5 7V16H19V12.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.5 3V9M18.5 9L16 6.5M18.5 9L21 6.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LockDesktop(
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
                d: "M10 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 6.42857C14 5.95518 14.3838 5.57143 14.8571 5.57143H19.1429C19.6162 5.57143 20 5.95518 20 6.42857V9.14286C20 9.61624 19.6162 10 19.1429 10H14.8571C14.3838 10 14 9.61624 14 9.14286V6.42857Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.7143 4.28571C15.7143 3.57563 16.2899 3 17 3C17.7101 3 18.2857 3.57563 18.2857 4.28571V5.57143H15.7143V4.28571Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FavouriteDesktop(
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
                d: "M11 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.5147 4.51469C14.8284 5.20098 14.8284 6.31368 15.5147 6.99997L18 9.48535L20.4853 7.00007C21.1716 6.31378 21.1716 5.20108 20.4853 4.51479C19.799 3.82851 18.6863 3.8285 18 4.51474C17.3138 3.82845 16.201 3.8284 15.5147 4.51469Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LaptopSettings(
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
                d: "M10 5H7C5.89543 5 5 5.89543 5 7V16H19V13.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.4641 5C18.2043 5 18.8506 5.40213 19.1964 5.99984M17.4641 5C16.7239 5 16.0776 5.40212 15.7318 5.99983M17.4641 5V3M17.4641 11V8.96675M20.9282 5L19.1964 5.99984M14 9L15.7318 8.00017M20.9282 9L19.1965 8.0002M14 5L15.7318 5.99983M15.7318 8.00017C15.5615 7.70596 15.4641 7.36436 15.4641 7C15.4641 6.63564 15.5615 6.29404 15.7318 5.99983M15.7318 8.00017C16.1255 8.68075 16.7977 8.98015 17.4641 8.96675M17.4641 8.96675C18.1521 8.95291 18.8338 8.60565 19.1965 8.0002M19.1965 8.0002C19.3645 7.71972 19.4641 7.38383 19.4641 7C19.4641 6.63565 19.3667 6.29405 19.1964 5.99984",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Desktop(
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
                x: "4",
                y: "4",
                width: "16",
                height: "12",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn DesktopSettings(
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
                d: "M10 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.4641 5C18.2043 5 18.8506 5.40213 19.1964 5.99984M17.4641 5C16.7239 5 16.0776 5.40212 15.7318 5.99983M17.4641 5V3M17.4641 11V8.96675M20.9282 5L19.1964 5.99984M14 9L15.7318 8.00017M20.9282 9L19.1965 8.0002M14 5L15.7318 5.99983M15.7318 8.00017C15.5615 7.70596 15.4641 7.36436 15.4641 7C15.4641 6.63564 15.5615 6.29404 15.7318 5.99983M15.7318 8.00017C16.1255 8.68075 16.7977 8.98015 17.4641 8.96675M17.4641 8.96675C18.1521 8.95291 18.8338 8.60565 19.1965 8.0002M19.1965 8.0002C19.3645 7.71972 19.4641 7.38383 19.4641 7C19.4641 6.63565 19.3667 6.29405 19.1964 5.99984",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CodeLaptop(
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
                d: "M10 5H7C5.89543 5 5 5.89543 5 7V16H19V11.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 4L13 6L15 8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 4L21 6L19 8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LockLaptop(
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
                d: "M11 5H7C5.89543 5 5 5.89543 5 7V16H19V13.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 6.42857C15 5.95518 15.3838 5.57143 15.8571 5.57143H20.1429C20.6162 5.57143 21 5.95518 21 6.42857V9.14286C21 9.61624 20.6162 10 20.1429 10H15.8571C15.3838 10 15 9.61624 15 9.14286V6.42857Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16.7143 4.28571C16.7143 3.57563 17.2899 3 18 3C18.7101 3 19.2857 3.57563 19.2857 4.28571V5.57143H16.7143V4.28571Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 16H21V17C21 18.1046 20.1046 19 19 19H5C3.89543 19 3 18.1046 3 17V16Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CodeDesktop(
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
                d: "M9 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 4L13 6L15 8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 4L21 6L19 8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn OfflineDesktop(
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
                d: "M9 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H18C19.1046 16 20 15.1046 20 14V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14.0251 4.02513C13.3918 4.6585 13 5.5335 13 6.5C13 8.433 14.567 10 16.5 10C17.4665 10 18.3415 9.60825 18.9749 8.97487M14.0251 4.02513C14.6585 3.39175 15.5335 3 16.5 3C18.433 3 20 4.567 20 6.5C20 7.4665 19.6082 8.3415 18.9749 8.97487M14.0251 4.02513L16.5 6.5L18.9749 8.97487",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M12 16V20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 20H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
