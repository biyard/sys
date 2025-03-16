use dioxus::prelude::*;
#[component]
pub fn DeletePhoto(
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
                d: "M13 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17 3L19 5M21 7L19 5M19 5L21 3M19 5L17 7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn PhotoOff(
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
                d: "M3 3L21 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M4.76761 3.6809C4.20533 4.05908 3.74601 4.57557 3.43598 5.18404C3.18869 5.66937 3.09012 6.18608 3.0442 6.74818C2.99998 7.28937 2.99999 7.95373 3 8.7587L3 8.8V15.2L3 15.2413C3 15.4903 3 15.7259 3.0013 15.9486C2.99932 15.9874 2.9996 16.0263 3.00212 16.0651C3.00584 16.5135 3.01607 16.9076 3.0442 17.2518C3.09012 17.8139 3.18869 18.3306 3.43598 18.816C3.81947 19.5686 4.43139 20.1805 5.18404 20.564C5.66938 20.8113 6.18608 20.9099 6.74818 20.9558C7.28937 21 7.95372 21 8.75868 21H8.8H15.2H15.2413C16.0463 21 16.7106 21 17.2518 20.9558C17.8139 20.9099 18.3306 20.8113 18.816 20.564C19.4242 20.2541 19.9406 19.795 20.3187 19.233L18.8447 17.7589C18.8251 17.8159 18.8041 17.8646 18.782 17.908C18.5903 18.2843 18.2843 18.5903 17.908 18.782C17.7516 18.8617 17.5274 18.9266 17.089 18.9624C16.6389 18.9992 16.0566 19 15.2 19H8.8C7.94343 19 7.36113 18.9992 6.91104 18.9624C6.47263 18.9266 6.24842 18.8617 6.09202 18.782C5.7157 18.5903 5.40974 18.2843 5.21799 17.908C5.1383 17.7516 5.07338 17.5274 5.03756 17.089C5.02093 16.8855 5.01167 16.6551 5.0065 16.3845L9.79344 11.2076L13.0858 14.5L15.3358 16.75C15.7263 17.1405 16.3595 17.1405 16.75 16.75C16.9681 16.5319 17.0644 16.2381 17.0389 15.9531L10.3809 9.29506C9.69594 9.0851 8.92053 9.25105 8.37868 9.7929C8.36946 9.80212 8.36043 9.81151 8.35158 9.82108L5 13.4456V8.8C5 7.94342 5.00078 7.36113 5.03756 6.91104C5.07338 6.47262 5.1383 6.24842 5.21799 6.09202C5.40974 5.7157 5.7157 5.40974 6.09202 5.21799C6.13545 5.19586 6.1841 5.17487 6.24112 5.15533L4.79289 3.70711C4.78427 3.69849 4.77584 3.68975 4.76761 3.6809ZM15.2594 11.3452C16.0445 10.7229 17.1887 10.7745 17.9143 11.5L19 12.5857V8.8C19 7.94342 18.9992 7.36113 18.9625 6.91104C18.9266 6.47262 18.8617 6.24842 18.782 6.09202C18.5903 5.7157 18.2843 5.40973 17.908 5.21799C17.7516 5.1383 17.5274 5.07337 17.089 5.03755C16.6389 5.00078 16.0566 5 15.2 5H8.91421L6.94455 3.03034C7.44781 2.99998 8.04853 2.99999 8.75869 3H8.75871H8.8H15.2H15.2413H15.2413C16.0463 2.99999 16.7106 2.99998 17.2518 3.04419C17.8139 3.09012 18.3306 3.18868 18.816 3.43598C19.5686 3.81947 20.1805 4.43139 20.564 5.18404C20.8113 5.66937 20.9099 6.18608 20.9558 6.74818C21 7.28937 21 7.95372 21 8.75868V8.8V14.9992V15.0007V15.2V15.2413C21 15.9515 21 16.5522 20.9697 17.0555L15.2594 11.3452Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn DownloadPhoto(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M13 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18 3V8M18 8L16 6M18 8L20 6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LockPhoto(
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
                d: "M12 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 6.42857C15 5.95518 15.3838 5.57143 15.8571 5.57143H20.1429C20.6162 5.57143 21 5.95518 21 6.42857V8.14286C21 8.61624 20.6162 9 20.1429 9H15.8571C15.3838 9 15 8.61624 15 8.14286V6.42857Z",
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
        }
    }
}
#[component]
pub fn UploadPhoto(
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
                d: "M13 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18 8V3M18 3L16 5M18 3L20 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Photo(
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
            rect {
                x: "4",
                y: "4",
                width: "16",
                height: "16",
                rx: "3",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Album(
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
                d: "M3 12.375L6.66789 8.70711C7.05842 8.31658 7.69158 8.31658 8.08211 8.70711L10.875 11.5M10.875 11.5L13.2304 9.1446C13.6209 8.75408 14.2541 8.75408 14.6446 9.14461L17 11.5M10.875 11.5L12.8438 13.4688",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn AddPhoto(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M13 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V11",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.5 3V5.5M18.5 8V5.5M18.5 5.5H16M18.5 5.5H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ExpandPhoto(
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
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
                cx: "8",
                cy: "4",
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
                cx: "12",
                cy: "4",
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
                cy: "8",
                r: "1",
                fill: "black",
            }
            rect {
                x: "4",
                y: "12",
                width: "8",
                height: "8",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 17L6.31788 15.4547C6.7145 15.1903 7.24262 15.2426 7.57969 15.5797L11 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
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
#[component]
pub fn FavouritePhoto(
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
                d: "M11 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.5147 3.51469C14.8284 4.20098 14.8284 5.31368 15.5147 5.99997L18 8.48535L20.4853 6.00007C21.1716 5.31378 21.1716 4.20108 20.4853 3.51479C19.799 2.82851 18.6863 2.8285 18 3.51474C17.3138 2.82845 16.201 2.8284 15.5147 3.51469Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CheckPhoto(
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
                d: "M12 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 6L17 8L20.5 4.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn RemovePhoto(
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
                d: "M12 4H8.8C7.11984 4 6.27976 4 5.63803 4.32698C5.07354 4.6146 4.6146 5.07354 4.32698 5.63803C4 6.27976 4 7.11984 4 8.8V15.2C4 16.8802 4 17.7202 4.32698 18.362C4.6146 18.9265 5.07354 19.3854 5.63803 19.673C6.27976 20 7.11984 20 8.8 20H15.2C16.8802 20 17.7202 20 18.362 19.673C18.9265 19.3854 19.3854 18.9265 19.673 18.362C20 17.7202 20 16.8802 20 15.2V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.8787 3.87868C15.3358 4.42157 15 5.17157 15 6C15 7.65685 16.3431 9 18 9C18.8284 9 19.5784 8.66421 20.1213 8.12132M15.8787 3.87868C16.4216 3.33579 17.1716 3 18 3C19.6569 3 21 4.34315 21 6C21 6.82843 20.6642 7.57843 20.1213 8.12132M15.8787 3.87868L18 6L20.1213 8.12132",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M4 16L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L13 15M13 15L15.7929 12.2071C16.1834 11.8166 16.8166 11.8166 17.2071 12.2071L20 15M13 15L15.25 17.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
