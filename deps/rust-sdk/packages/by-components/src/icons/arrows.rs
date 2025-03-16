use dioxus::prelude::*;
#[component]
pub fn ArrowMove(
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
                d: "M12 9V3.34315M12 3.34315L9.17157 6.17157M12 3.34315L14.8284 6.17157",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 12H20.6569M20.6569 12L17.8284 9.17157M20.6569 12L17.8284 14.8284",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 12H3.34315M3.34315 12L6.17157 14.8284M3.34315 12L6.17157 9.17157",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 15V20.6569M12 20.6569L14.8284 17.8284M12 20.6569L9.17157 17.8284",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowMerge(
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
                d: "M3 7V12M3 17V12M3 12H10M10 12L7 9M10 12L7 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M21 7V12M21 17V12M21 12H14M14 12L17 9M14 12L17 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ChevronUp(
    #[props(default = "24".to_string())] width: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M5 15L11.2929 8.70711C11.6834 8.31658 12.3166 8.31658 12.7071 8.70711L19 15",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn UpDown(
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
                d: "M14 6L14 18L18 14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10 18L10 6L6 10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ChevronDown(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = "24".to_string())] height: String,
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
                d: "M19 9L12.7071 15.2929C12.3166 15.6834 11.6834 15.6834 11.2929 15.2929L5 9",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowDownRight(
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
                d: "M6.34312 6.34317L17.6568 17.6569M17.6568 17.6569L9.17154 17.6569M17.6568 17.6569L17.6568 9.1716",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowRight1(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M8 12L16 12M16 12L13 15M16 12L13 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LineArrowLeft(
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
                d: "M13 19L6.70711 12.7071C6.31658 12.3166 6.31658 11.6834 6.70711 11.2929L13 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M18 19L18 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn SquareArrowLeft(
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
            rect {
                x: "4",
                y: "4",
                width: "16",
                height: "16",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16 12L8 12M8 12L11 15M8 12L11 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn DoubleArrowDown(
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
                d: "M19 5L12.7071 11.2929C12.3166 11.6834 11.6834 11.6834 11.2929 11.2929L5 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 13L12.7071 19.2929C12.3166 19.6834 11.6834 19.6834 11.2929 19.2929L5 13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowUp(
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
            path {
                d: "M12 20L12 4M12 4L18 10M12 4L6 10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowsHorizontalExpand(
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
                d: "M21 7V17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 7V17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 12H17M7 12L9.5 9.5M7 12L9.5 14.5M17 12L14.5 9.5M17 12L14.5 14.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn DoubleArrowRight(
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
                d: "M5.5 5L11.7929 11.2929C12.1834 11.6834 12.1834 12.3166 11.7929 12.7071L5.5 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M13.5 5L19.7929 11.2929C20.1834 11.6834 20.1834 12.3166 19.7929 12.7071L13.5 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowDiagonal(
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
                d: "M5 5L19 19M5 5L10 5M5 5L5 10M19 19V14M19 19H14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ShuffleArrow(
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
            path {
                d: "M4 8H6.8915C7.58108 8 8.22202 8.35524 8.5875 8.94L12.4125 15.06C12.778 15.6448 13.4189 16 14.1085 16H20M20 16L17.5 13.5M20 16L17.5 18.5M14 8H20M20 8L17.5 10.5M20 8L17.5 5.5M4 16H8",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowExpand(
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
                d: "M10 10L5 5M5 5L5 10M5 5L10 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 14L19 19M19 19V14M19 19H14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowUpRight(
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
                d: "M6.34319 17.6569L17.6569 6.34315M17.6569 6.34315L17.6569 14.8284M17.6569 6.34315L9.17161 6.34315",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowResizeVertical(
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
                d: "M8 12L12 12M16 12L12 12M12 12L12 4M12 12L12 20M12 4L9.5 6.5M12 4L14.5 6.5M12 20L14.5 17.5M12 20L9.5 17.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowHorizontal(
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
                d: "M20 12L4 12M20 12L16.5 15.5M20 12L16.5 8.5M4 12L7.5 8.5M4 12L7.5 15.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowDown1(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M12 8V16M12 16L9 13M12 16L15 13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BendArrowUp(
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
                d: "M6 19L10 19C12.2091 19 14 17.2091 14 15L14 5M14 5L18 9M14 5L10 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn TrendingUp(
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
                d: "M3 17L8.29289 11.7071C8.68342 11.3166 9.31658 11.3166 9.70711 11.7071L12.2929 14.2929C12.6834 14.6834 13.3166 14.6834 13.7071 14.2929L21 7M21 7H16M21 7V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowUp(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                transform: "rotate(-180 12 12)",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 17L12 7.00005M12 7.00005L14.5 9.50005M12 7.00005L9.5 9.50005",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Exchange(
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
                d: "M8 10H20L16 6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16 14L4 14L8 18",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BendArrowLeft(
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
                d: "M19 18V14C19 11.7909 17.2091 10 15 10H5M5 10L9 6M5 10L9 14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowsVerticalExpand(
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
                d: "M17 21L7 21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17 3L7 3",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 7L12 17M12 7L14.5 9.5M12 7L9.5 9.5M12 17L14.5 14.5M12 17L9.5 14.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LineArrowDown(
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
                d: "M19 11L12.7071 17.2929C12.3166 17.6834 11.6834 17.6834 11.2929 17.2929L5 11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 6L5 6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowRightDown(
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
                d: "M7 20L4 17M7 20L10 17M7 20V11C7 8.79086 8.79086 7 11 7H20M20 7L17 4M20 7L17 10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowRight(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                transform: "rotate(-90 12 12)",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7 12H17M17 12L14.5 14.5M17 12L14.5 9.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowLeftDown(
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
                d: "M4 7L7 4M4 7L7 10M4 7L13 7C15.2091 7 17 8.79086 17 11L17 20M17 20L20 17M17 20L14 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ChevronRight(
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
                d: "M9.5 5L15.7929 11.2929C16.1834 11.6834 16.1834 12.3166 15.7929 12.7071L9.5 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowRight(
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
                d: "M4 12L20 12M20 12L14 18M20 12L14 6",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowLeftUp(
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
                d: "M17 4L20 7M17 4L14 7M17 4L17 13C17 15.2091 15.2091 17 13 17L4 17M4 17L7 20M4 17L7 14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowCompress1(
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
                d: "M5 5L9 9M9 9V5M9 9H5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 5L15 9M15 9L19 9M15 9L15 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5 19L9 15M9 15L5 15M9 15L9 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 19L15 15M15 15L15 19M15 15L19 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LineArrowRight(
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
                d: "M11 5L17.2929 11.2929C17.6834 11.6834 17.6834 12.3166 17.2929 12.7071L11 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6 5L6 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn SquareArrowRight(
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
            rect {
                x: "4",
                y: "4",
                width: "16",
                height: "16",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M8 12L16 12M16 12L13 15M16 12L13 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn DoubleArrowUp(
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
                d: "M5 19L11.2929 12.7071C11.6834 12.3166 12.3166 12.3166 12.7071 12.7071L19 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5 11L11.2929 4.70711C11.6834 4.31658 12.3166 4.31658 12.7071 4.70711L19 11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn DoubleArrowLeft(
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
                d: "M19 19L12.7071 12.7071C12.3166 12.3166 12.3166 11.6834 12.7071 11.2929L19 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M11 19L4.70711 12.7071C4.31658 12.3166 4.31658 11.6834 4.70711 11.2929L11 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CrossoverArrowsDown(
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
                d: "M5 5L19 19M19 19L15 19M19 19L19 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 5L15.5 8.5L14.625 9.375M5 19L5 15M5 19L9 19M5 19L8.5 15.5L9.375 14.625",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn LineArrowUp(
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
                d: "M5 13L11.2929 6.70711C11.6834 6.31658 12.3166 6.31658 12.7071 6.70711L19 13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5 18H19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowLeft(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                transform: "rotate(90 12 12)",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17 12L7 12M7 12L9.5 9.5M7 12L9.5 14.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CrossoverArrowsRight(
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
                d: "M5 19L19 5M19 5L19 9M19 5L15 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5 5L8.5 8.5L9.375 9.375M19 19L15 19M19 19L19 15M19 19L15.5 15.5L14.625 14.625",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowVertical(
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
                d: "M12 4V20M12 4L15.5 7.5M12 4L8.5 7.5M12 20L8.5 16.5M12 20L15.5 16.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowDown(
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
                d: "M12 7V17M12 17L9.5 14.5M12 17L14.5 14.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BendArrowRight(
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
                d: "M5 6L5 10C5 12.2091 6.79086 14 9 14L19 14M19 14L15 18M19 14L15 10",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ChevronLeft(
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
                d: "M15.5 19L9.20711 12.7071C8.81658 12.3166 8.81658 11.6834 9.20711 11.2929L15.5 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowDivide(
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
                d: "M10 17L10 12M10 7L10 12M10 12L3 12M3 12L6 15M3 12L6 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 17L14 12M14 7L14 12M14 12L21 12M21 12L18 15M21 12L18 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowDown(
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
                d: "M12 4L12 20M12 20L6 14M12 20L18 14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowLeft(
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
                d: "M20 12H4M4 12L10 6M4 12L10 18",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ExchangeDiagonal(
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
                d: "M7.75738 13.4142L16.2427 4.92893H10.5858",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16.2426 10.5858L7.75734 19.0711L13.4142 19.0711",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowUpLeft(
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
                d: "M17.6569 17.6569L6.34317 6.34317M6.34317 6.34317H14.8285M6.34317 6.34317V14.8285",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn SquareArrowUp(
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
            rect {
                x: "4",
                y: "4",
                width: "16",
                height: "16",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 16L12 8M12 8L15 11M12 8L9 11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowExpand1(
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
                d: "M9 9L5 5M5 5L5 9M5 5L9 5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 9L19 5M19 5L15 5M19 5L19 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 15L5 19M5 19L9 19M5 19L5 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 15L19 19M19 19L19 15M19 19L15 19",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowLeft1(
    #[props(default = "24".to_string())] height: String,
    #[props(default = "none".to_string())] fill: String,
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M16 12L8 12M8 12L11 9M8 12L11 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowCompress(
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
                d: "M5 5L10 10M10 10V5M10 10H5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 19L14 14M14 14L14 19M14 14L19 14",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowDownLeft(
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
                d: "M17.6568 6.34317L6.3431 17.6569M6.3431 17.6569V9.17159M6.3431 17.6569H14.8284",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CircleArrowUp1(
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
            circle {
                cx: "12",
                cy: "12",
                r: "9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path {
                d: "M12 16L12 8M12 8L15 11M12 8L9 11",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CrossoverArrowsLeft(
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
                d: "M19 5L5 19M5 19V15M5 19H9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 19L15.5 15.5L14.625 14.625M5 5L9 5M5 5L5 9M5 5L8.5 8.5L9.375 9.375",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CrossoverArrowsUp(
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
                d: "M19 19L5 5M5 5L9 5M5 5L5 9",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5 19L8.5 15.5L9.375 14.625M19 5L19 9M19 5L15 5M19 5L15.5 8.5L14.625 9.375",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn SquareArrowDown(
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
            rect {
                x: "4",
                y: "4",
                width: "16",
                height: "16",
                rx: "2",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 8V16M12 16L9 13M12 16L15 13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowRightUp(
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
                d: "M20 17L17 20M20 17L17 14M20 17L11 17C8.79086 17 7 15.2091 7 13L7 4M7 4L4 7M7 4L10 7",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn ArrowResizeHorizontal(
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
            path {
                d: "M12 16L12 12M12 8L12 12M12 12L4 12M12 12L20 12M4 12L6.5 14.5M4 12L6.5 9.5M20 12L17.5 9.5M20 12L17.5 14.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn TrendingDown(
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
                d: "M3 7L8.29289 12.2929C8.68342 12.6834 9.31658 12.6834 9.70711 12.2929L12.2929 9.70711C12.6834 9.31658 13.3166 9.31658 13.7071 9.70711L21 17M21 17V12M21 17H16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn BendArrowDown(
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
                d: "M18 5L14 5C11.7909 5 10 6.79086 10 9L10 19M10 19L6 15M10 19L14 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
