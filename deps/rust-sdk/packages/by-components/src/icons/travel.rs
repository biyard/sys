use dioxus::prelude::*;
#[component]
pub fn MyLocation(
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
            circle {
                cx: "12",
                cy: "12",
                r: "2.25",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "6.75",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 5.25V3",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18.75 12H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 18.75V21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5.25 12H3",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FlightTakeoff(
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
                d: "M11.4216 5.34681L11.8885 4.46251C11.6388 4.33065 11.3449 4.31053 11.0796 4.40712L11.4216 5.34681ZM8.60251 6.37287L8.26049 5.43318C7.96713 5.53995 7.74047 5.7776 7.64769 6.07568C7.55491 6.37376 7.60666 6.69806 7.7876 6.95247L8.60251 6.37287ZM15.3736 7.43348L14.9067 8.31778C15.1757 8.45982 15.4948 8.47168 15.7736 8.35001L15.3736 7.43348ZM21.0969 9.27453L20.7549 8.33483L21.0969 9.27453ZM7.70772 14.1478L7.02019 14.8739C7.29524 15.1344 7.69381 15.217 8.04974 15.0875L7.70772 14.1478ZM6.89589 13.3791L7.58342 12.653C7.30837 12.3926 6.9098 12.3099 6.55387 12.4394L6.89589 13.3791ZM6.89587 13.3791L6.20834 14.1053C6.48338 14.3657 6.88196 14.4484 7.23789 14.3188L6.89587 13.3791ZM5.27212 11.8417L5.95966 11.1156L5.95966 11.1156L5.27212 11.8417ZM3.6484 10.3044L3.30638 9.36468C2.9807 9.48322 2.73962 9.76193 2.66924 10.1013C2.59885 10.4406 2.70919 10.7922 2.96086 11.0305L3.6484 10.3044ZM6.46747 9.27831L7.15501 8.55216C6.87996 8.29174 6.48138 8.20907 6.12545 8.33862L6.46747 9.27831ZM7.94358 10.6759L7.25605 11.4021C7.5475 11.678 7.9757 11.753 8.34356 11.5924L7.94358 10.6759ZM10.782 9.43723L11.182 10.3538C11.4581 10.2333 11.6651 9.99476 11.7456 9.70449C11.8262 9.41421 11.7715 9.10311 11.5969 8.85764L10.782 9.43723ZM11.0796 4.40712L8.26049 5.43318L8.94453 7.31256L11.7636 6.2865L11.0796 4.40712ZM15.8405 6.54918L11.8885 4.46251L10.9547 6.23111L14.9067 8.31778L15.8405 6.54918ZM16.1296 6.01247L14.9736 6.51695L15.7736 8.35001L16.9296 7.84553L16.1296 6.01247ZM22.1869 8.61011C21.2949 6.1593 18.52 4.96931 16.1296 6.01247L16.9296 7.84553C18.2626 7.26379 19.8101 7.92741 20.3075 9.29415L22.1869 8.61011ZM21.4389 10.2142C22.0884 9.97781 22.4233 9.25963 22.1869 8.61011L20.3075 9.29415C20.1662 8.90572 20.3664 8.47621 20.7549 8.33483L21.4389 10.2142ZM8.04974 15.0875L21.4389 10.2142L20.7549 8.33483L7.3657 13.2081L8.04974 15.0875ZM6.20836 14.1053L7.02019 14.8739L8.39525 13.4216L7.58342 12.653L6.20836 14.1053ZM6.55387 12.4394L6.55385 12.4394L7.23789 14.3188L7.23791 14.3188L6.55387 12.4394ZM7.58341 12.653L5.95966 11.1156L4.58459 12.5679L6.20834 14.1053L7.58341 12.653ZM5.95966 11.1156L4.33593 9.57822L2.96086 11.0305L4.58459 12.5679L5.95966 11.1156ZM3.99042 11.2441L6.80949 10.218L6.12545 8.33862L3.30638 9.36468L3.99042 11.2441ZM5.77994 10.0045L7.25605 11.4021L8.63112 9.94977L7.15501 8.55216L5.77994 10.0045ZM10.3821 8.52071L7.54361 9.75939L8.34356 11.5924L11.182 10.3538L10.3821 8.52071ZM7.7876 6.95247L9.96712 10.0168L11.5969 8.85764L9.41741 5.79327L7.7876 6.95247Z",
                fill: "black",
            }
            path {
                d: "M4 19L20 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Flag(
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
                d: "M5 20V14M5 14V4H19L15 9L19 14H5Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn FlightLanding(
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
                d: "M13.8156 5.68884L14.7417 5.31155C14.6352 5.05002 14.423 4.84573 14.1576 4.74914L13.8156 5.68884ZM10.9965 4.66278L11.3386 3.72308C11.0452 3.61631 10.7188 3.65266 10.4561 3.82137C10.1935 3.99007 10.0247 4.27177 9.99974 4.58296L10.9965 4.66278ZM15.5018 9.82763L14.5757 10.2049C14.6904 10.4866 14.9272 10.7008 15.219 10.7868L15.5018 9.82763ZM18.7027 14.9168L18.3606 15.8565L18.7027 14.9168ZM5.31348 10.0436L4.32004 10.1579C4.36335 10.5342 4.61553 10.8537 4.97146 10.9832L5.31348 10.0436ZM5.18566 8.93289L6.1791 8.81856C6.1358 8.44227 5.88361 8.12275 5.52768 7.9932L5.18566 8.93289ZM5.18565 8.93289L4.1922 9.04721C4.23551 9.4235 4.48769 9.74303 4.84363 9.87258L5.18565 8.93289ZM4.93 6.71145L5.92345 6.59712L5.92345 6.59712L4.93 6.71145ZM4.67436 4.49004L5.01638 3.55035C4.6907 3.43181 4.32687 3.49036 4.05482 3.70508C3.78277 3.91979 3.64129 4.26007 3.68092 4.60437L4.67436 4.49004ZM7.49344 5.5161L8.48688 5.40178C8.44358 5.02549 8.19139 4.70596 7.83546 4.57641L7.49344 5.5161ZM7.72584 7.53556L6.73239 7.64989C6.77828 8.04861 7.05812 8.38128 7.4431 8.49476L7.72584 7.53556ZM10.6964 8.41118L10.4137 9.37038C10.7026 9.45555 11.0146 9.40593 11.2628 9.23532C11.5111 9.0647 11.6692 8.79127 11.6932 8.49099L10.6964 8.41118ZM14.1576 4.74914L11.3386 3.72308L10.6545 5.60247L13.4736 6.62853L14.1576 4.74914ZM16.4279 9.45034L14.7417 5.31155L12.8895 6.06613L14.5757 10.2049L16.4279 9.45034ZM16.9943 9.22505L15.7845 8.86843L15.219 10.7868L16.4289 11.1434L16.9943 9.22505ZM19.9647 15.1085C20.8568 12.6577 19.496 9.96245 16.9943 9.22505L16.4289 11.1434C17.824 11.5547 18.5828 13.0577 18.0854 14.4245L19.9647 15.1085ZM18.3606 15.8565C19.0102 16.0929 19.7283 15.758 19.9647 15.1085L18.0854 14.4245C18.2267 14.036 18.6562 13.8357 19.0447 13.9771L18.3606 15.8565ZM4.97146 10.9832L18.3606 15.8565L19.0447 13.9771L5.6555 9.10386L4.97146 10.9832ZM4.19222 9.04722L4.32004 10.1579L6.30693 9.92922L6.1791 8.81856L4.19222 9.04722ZM5.52768 7.9932L5.52767 7.99319L4.84363 9.87258L4.84364 9.87259L5.52768 7.9932ZM6.17909 8.81856L5.92345 6.59712L3.93656 6.82577L4.1922 9.04721L6.17909 8.81856ZM5.92345 6.59712L5.6678 4.37571L3.68092 4.60437L3.93656 6.82577L5.92345 6.59712ZM4.33234 5.42973L7.15142 6.4558L7.83546 4.57641L5.01638 3.55035L4.33234 5.42973ZM6.49999 5.63043L6.73239 7.64989L8.71928 7.42123L8.48688 5.40178L6.49999 5.63043ZM10.9792 7.45198L8.00858 6.57636L7.4431 8.49476L10.4137 9.37038L10.9792 7.45198ZM9.99974 4.58296L9.69961 8.33137L11.6932 8.49099L11.9934 4.74259L9.99974 4.58296Z",
                fill: "black",
            }
            path {
                d: "M4 19L20 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn TravelBag(
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
                d: "M8 6C8 4.89543 8.89543 4 10 4H14C15.1046 4 16 4.89543 16 6V20H8V6Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            rect {
                x: "3",
                y: "8",
                width: "18",
                height: "12",
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
pub fn Compass(
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
                d: "M16 8L10 10L8 16L14 14L16 8Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Map(
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
                d: "M3 19.382V5.72076C3 5.29033 3.27543 4.90819 3.68377 4.77208L8.36754 3.21082C8.77808 3.07397 9.22192 3.07397 9.63246 3.21082L14.3675 4.78918C14.7781 4.92603 15.2219 4.92603 15.6325 4.78918L19.6838 3.43874C20.3313 3.2229 21 3.70487 21 4.38743V17.382C21 17.7607 20.786 18.107 20.4472 18.2764L15.8944 20.5528C15.3314 20.8343 14.6686 20.8343 14.1056 20.5528L9.89443 18.4472C9.33137 18.1657 8.66863 18.1657 8.10557 18.4472L4.44721 20.2764C3.78231 20.6088 3 20.1253 3 19.382Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 5.00006V20.5001",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 4L9 18",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Globe(
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
            circle {
                cx: "12.5",
                cy: "9.5",
                r: "4.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.3209 15.2453C16.5664 15.8784 15.6946 16.3567 14.7553 16.6529C13.816 16.949 12.8275 17.0573 11.8463 16.9715C10.8652 16.8856 9.91051 16.6074 9.03689 16.1526C8.16326 15.6978 7.38776 15.0754 6.75467 14.3209C6.12158 13.5664 5.64329 12.6946 5.34712 11.7553C5.05095 10.816 4.9427 9.8275 5.02854 8.84633C5.11438 7.86516 5.39264 6.91051 5.84742 6.03688C6.3022 5.16326 6.92461 4.38776 7.67909 3.75467",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 17V20M12 20H9.5M12 20C12 20 12.8284 20 14 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LocationPoint(
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
            circle {
                cx: "12",
                cy: "10",
                r: "3",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 9.75C19 15.375 12 21 12 21C12 21 5 15.375 5 9.75C5 6.02208 8.13401 3 12 3C15.866 3 19 6.02208 19 9.75Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Earth(
    #[props(default = "".to_string())]
    class: String,
    #[props(default = "24".to_string())]
    height: String,
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M13 3.0481C12.652 3.34099 12.3384 3.68664 12.0717 4.08265C10.5289 6.3729 11.1348 9.48019 13.4251 11.023C13.6058 11.1448 13.7917 11.2532 13.9815 11.3484C16 12.3612 16.7706 10.9962 17.8634 11.7323C18.5723 12.2099 18.7598 13.1716 18.2823 13.8805C17.7819 14.6234 17 15 17.1352 16.2168C17.2116 16.9036 17.6335 17.4954 18.1802 18",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 9.28003C5.08188 9.54331 6.083 10.1678 6.80601 11.1259C7.57668 12.1472 7.89615 13.3684 7.79785 14.5504C7.74578 15.1764 8.15405 15.8083 8.6794 16.1526C8.93324 16.319 9.16323 16.5312 9.35655 16.7874C10.2126 17.9218 10.0418 19.511 9 20.4407",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Navigation(
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
                d: "M4.47046 17.0591L10.2111 5.57771C10.9482 4.10361 13.0518 4.10362 13.7889 5.57771L19.5295 17.0591C20.3661 18.7322 18.6528 20.5356 16.9391 19.7858L12.4008 17.8004C12.1453 17.6886 11.8547 17.6886 11.5992 17.8004L7.06094 19.7858C5.34719 20.5356 3.6339 18.7322 4.47046 17.0591Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
