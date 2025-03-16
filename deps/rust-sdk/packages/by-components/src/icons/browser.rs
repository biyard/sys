use dioxus::prelude::*;
#[component]
pub fn LockBrowser(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "24".to_string())]
    width: String,
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
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H10M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21M21 10V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14 17.4286C14 16.9552 14.3838 16.5714 14.8571 16.5714H19.1429C19.6162 16.5714 20 16.9552 20 17.4286V20.1429C20 20.6162 19.6162 21 19.1429 21H14.8571C14.3838 21 14 20.6162 14 20.1429V17.4286Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.7143 15.2857C15.7143 14.5756 16.2899 14 17 14C17.7101 14 18.2857 14.5756 18.2857 15.2857V16.5714H15.7143V15.2857Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn BrowserCode(
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
            path {
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H9M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21M21 10V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 16L13 18L15 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M19 16L21 18L19 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn UploadBrowser(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "24".to_string())]
    width: String,
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
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H12M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21M21 10V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.5 21L17.5 15M17.5 15L20 17.5M17.5 15L15 17.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn BrowserSettings(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "24".to_string())]
    width: String,
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
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H11M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21M21 10V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
            path {
                d: "M18 15.75C18.641 15.75 19.2007 16.1019 19.5002 16.6249M18 15.75C17.3589 15.75 16.7992 16.1019 16.4997 16.6249M18 15.75V14M18 21V19.2209M21 15.75L19.5002 16.6249M15 19.25L16.4997 18.3751M21 19.25L19.5003 18.3752M15 15.75L16.4997 16.6249M16.4997 18.3751C16.3523 18.1177 16.2679 17.8188 16.2679 17.5C16.2679 17.1812 16.3523 16.8823 16.4997 16.6249M16.4997 18.3751C16.8408 18.9707 17.4229 19.2326 18 19.2209M18 19.2209C18.5958 19.2088 19.1862 18.9049 19.5003 18.3752M19.5003 18.3752C19.6458 18.1298 19.732 17.8358 19.732 17.5C19.732 17.1812 19.6477 16.8823 19.5002 16.6249",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn AddBrowser(
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
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H11M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21M21 10V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17 14V17M17 20V17M17 17H14M17 17H20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn DownloadBrowser(
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
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H12M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21M21 10V13",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M17.5 15L17.5 21M17.5 21L20 18.5M17.5 21L15 18.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Browser(
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
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H19C20.1046 20 21 19.1046 21 18V10M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn RemoveBrowser(
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
                d: "M3 10V18C3 19.1046 3.89543 20 5 20H12M3 10V6C3 4.89543 3.89543 4 5 4H19C20.1046 4 21 4.89543 21 6V10M3 10H21M21 10V12",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15.8787 14.8787C15.3358 15.4216 15 16.1716 15 17C15 18.6569 16.3431 20 18 20C18.8284 20 19.5784 19.6642 20.1213 19.1213M15.8787 14.8787C16.4216 14.3358 17.1716 14 18 14C19.6569 14 21 15.3431 21 17C21 17.8284 20.6642 18.5784 20.1213 19.1213M15.8787 14.8787L18 17L20.1213 19.1213",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            circle {
                cx: "6",
                cy: "7",
                r: "1",
                fill: "black",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "1",
                fill: "black",
            }
        }
    }
}
