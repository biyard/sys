use dioxus::prelude::*;
#[component]
pub fn DownloadCloud1(
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
                d: "M12 10V20M12 20L9.5 17.5M12 20L14.5 17.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M6.3218 7.05726C7.12925 4.69709 9.36551 3 12 3C14.6345 3 16.8708 4.69709 17.6782 7.05726C19.5643 7.37938 21 9.02203 21 11C21 13.2091 19.2091 15 17 15H16C15.4477 15 15 14.5523 15 14C15 13.4477 15.4477 13 16 13H17C18.1046 13 19 12.1046 19 11C19 9.89543 18.1046 9 17 9C16.9776 9 16.9552 9.00037 16.9329 9.0011C16.4452 9.01702 16.0172 8.67854 15.9202 8.20023C15.5502 6.37422 13.9345 5 12 5C10.0655 5 8.44979 6.37422 8.07977 8.20023C7.98284 8.67854 7.55482 9.01702 7.06706 9.0011C7.04476 9.00037 7.02241 9 7 9C5.89543 9 5 9.89543 5 11C5 12.1046 5.89543 13 7 13H8C8.55228 13 9 13.4477 9 14C9 14.5523 8.55228 15 8 15H7C4.79086 15 3 13.2091 3 11C3 9.02203 4.43567 7.37938 6.3218 7.05726Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn Cloud(
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24".to_string())]
    width: String,
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
                d: "M16.9776 11L15.9816 10.9102C15.9564 11.1899 16.05 11.4673 16.2394 11.6746C16.4289 11.8819 16.6967 12 16.9776 12V11ZM8.97071 18.9999L8.97785 17.9999H8.97071V18.9999ZM6 10.5142L6.19893 11.4942C6.66596 11.3994 7.0012 10.9882 7 10.5117L6 10.5142ZM17.9735 11.0898C17.9911 10.8953 18 10.6985 18 10.5H16C16 10.6385 15.9938 10.7754 15.9816 10.9102L17.9735 11.0898ZM17 10H16.9776V12H17V10ZM22 15C22 12.2386 19.7614 10 17 10V12C18.6568 12 20 13.3431 20 15H22ZM17 20C19.7614 20 22 17.7614 22 15H20C20 16.6569 18.6568 18 17 18V20ZM8.99998 20H17V18H8.99998V20ZM8.96357 19.9999C8.97567 20 8.98781 20 8.99998 20V18C8.99263 18 8.98525 18 8.97785 17.9999L8.96357 19.9999ZM8.97071 17.9999H6.85713V19.9999H8.97071V17.9999ZM6.85713 17.9999C5.04248 17.9999 3.57141 16.5288 3.57141 14.7142H1.57141C1.57141 17.6334 3.93791 19.9999 6.85713 19.9999V17.9999ZM3.57141 14.7142C3.57141 13.1257 4.69958 11.7986 6.19893 11.4942L5.80108 9.53418C3.38799 10.024 1.57141 12.1558 1.57141 14.7142H3.57141ZM4.99998 10.5C4.99998 10.5056 4.99999 10.5111 5.00001 10.5167L7 10.5117C6.99999 10.5078 6.99998 10.5039 6.99998 10.5H4.99998ZM11.5 4C7.91013 4 4.99998 6.91015 4.99998 10.5H6.99998C6.99998 8.01472 9.0147 6 11.5 6V4ZM18 10.5C18 6.91015 15.0898 4 11.5 4V6C13.9853 6 16 8.01472 16 10.5H18Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn DownloadCloud(
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
                d: "M17 19C19.2091 19 21 17.2091 21 15C21 12.7909 19.2091 11 17 11H16.9776C16.9924 10.8353 17 10.6685 17 10.5C17 7.46243 14.5376 5 11.5 5C8.46242 5 5.99998 7.46243 5.99998 10.5C5.99998 10.5047 5.99999 10.5095 6 10.5142C4.04379 10.9113 2.57141 12.6408 2.57141 14.7142C2.57141 17.0811 4.49019 18.9999 6.85713 18.9999",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 14L12 19M12 19L14 17M12 19L10 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn CloudOff(
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
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M6.94688 5.86111C5.91346 6.87554 5.21752 8.23258 5.04298 9.74832C3.01743 10.4886 1.57141 12.4316 1.57141 14.7142C1.57141 17.6334 3.93791 19.9999 6.85713 19.9999H8.97071L8.99998 20H17C18.1477 20 19.205 19.6133 20.0489 18.9631L18.6146 17.5289C18.1485 17.8271 17.5944 18 17 18H8.99998L8.97785 17.9999V17.9999H8.97071H6.85713C5.04248 17.9999 3.57141 16.5288 3.57141 14.7142C3.57141 13.1257 4.69958 11.7986 6.19893 11.4942C6.66596 11.3994 7.0012 10.9882 7 10.5117L6.99998 10.5C6.99998 9.23554 7.52151 8.09289 8.36118 7.27541L6.94688 5.86111ZM19.8509 15.9367C19.9476 15.642 20 15.3271 20 15C20 13.3431 18.6568 12 17 12H16.9776C16.6967 12 16.4289 11.8819 16.2394 11.6746C16.05 11.4673 15.9564 11.1899 15.9816 10.9102C15.9938 10.7754 16 10.6385 16 10.5C16 8.01472 13.9853 6 11.5 6C11.0213 6 10.5601 6.07474 10.1274 6.21318L8.59692 4.68272C9.47069 4.24583 10.4566 4 11.5 4C14.9547 4 17.7799 6.69515 17.9877 10.0975C20.2761 10.5561 22 12.5767 22 15C22 15.889 21.768 16.7237 21.3613 17.4471L19.8509 15.9367Z",
                fill: "black",
            }
            path {
                d: "M4 4L20 20",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn UploadCloud(
    #[props(default = "24".to_string())]
    height: String,
    #[props(default = "none".to_string())]
    fill: String,
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
                d: "M17 19C19.2091 19 21 17.2091 21 15C21 12.7909 19.2091 11 17 11H16.9776C16.9924 10.8353 17 10.6685 17 10.5C17 7.46243 14.5376 5 11.5 5C8.46242 5 5.99998 7.46243 5.99998 10.5C5.99998 10.5047 5.99999 10.5095 6 10.5142C4.04379 10.9113 2.57141 12.6408 2.57141 14.7142C2.57141 17.0811 4.49019 18.9999 6.85713 18.9999",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 19L12 14M12 14L14 16M12 14L10 16",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn UploadCloud1(
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
                d: "M12 19L12 9M12 9L14.5 11.5M12 9L9.5 11.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M6.3218 7.05726C7.12925 4.69709 9.36551 3 12 3C14.6345 3 16.8708 4.69709 17.6782 7.05726C19.5643 7.37938 21 9.02203 21 11C21 13.2091 19.2091 15 17 15H16C15.4477 15 15 14.5523 15 14C15 13.4477 15.4477 13 16 13H17C18.1046 13 19 12.1046 19 11C19 9.89543 18.1046 9 17 9C16.9776 9 16.9552 9.00037 16.9329 9.0011C16.4452 9.01702 16.0172 8.67854 15.9202 8.20023C15.5502 6.37422 13.9345 5 12 5C10.0655 5 8.44979 6.37422 8.07977 8.20023C7.98284 8.67854 7.55482 9.01702 7.06706 9.0011C7.04476 9.00037 7.02241 9 7 9C5.89543 9 5 9.89543 5 11C5 12.1046 5.89543 13 7 13H8C8.55228 13 9 13.4477 9 14C9 14.5523 8.55228 15 8 15H7C4.79086 15 3 13.2091 3 11C3 9.02203 4.43567 7.37938 6.3218 7.05726Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn CloudSettings(
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
                d: "M6 18.9142C4.04379 18.5171 2.57141 16.7876 2.57141 14.7142C2.57141 12.6408 4.04379 10.9113 6 10.5142C5.99999 10.5095 5.99998 10.5047 5.99998 10.5C5.99998 7.46243 8.46242 5 11.5 5C14.5376 5 17 7.46243 17 10.5C17 10.6685 16.9924 10.8353 16.9776 11H17C19.2091 11 21 12.7909 21 15C21 16.8638 19.7252 18.4299 18 18.874",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 14.75C12.641 14.75 13.2007 15.1019 13.5002 15.6249M12 14.75C11.3589 14.75 10.7992 15.1019 10.4997 15.6249M12 14.75V13M12 20V18.2209M15 14.75L13.5002 15.6249M9 18.25L10.4997 17.3751M15 18.25L13.5003 17.3752M9 14.75L10.4997 15.6249M10.4997 17.3751C10.3523 17.1177 10.2679 16.8188 10.2679 16.5C10.2679 16.1812 10.3523 15.8823 10.4997 15.6249M10.4997 17.3751C10.8408 17.9707 11.4229 18.2326 12 18.2209M12 18.2209C12.5958 18.2088 13.1862 17.9049 13.5003 17.3752M13.5003 17.3752C13.6458 17.1298 13.732 16.8358 13.732 16.5C13.732 16.1812 13.6477 15.8823 13.5002 15.6249",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
#[component]
pub fn LockCloud(
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
                d: "M6 18.9142C4.04379 18.5171 2.57141 16.7876 2.57141 14.7142C2.57141 12.6408 4.04379 10.9113 6 10.5142C5.99999 10.5095 5.99998 10.5047 5.99998 10.5C5.99998 7.46243 8.46242 5 11.5 5C14.5376 5 17 7.46243 17 10.5C17 10.6685 16.9924 10.8353 16.9776 11H17C19.2091 11 21 12.7909 21 15C21 16.8638 19.7252 18.4299 18 18.874",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M9 16.4286C9 15.9552 9.38376 15.5714 9.85714 15.5714H14.1429C14.6162 15.5714 15 15.9552 15 16.4286V19.1429C15 19.6162 14.6162 20 14.1429 20H9.85714C9.38376 20 9 19.6162 9 19.1429V16.4286Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10.7143 14.2857C10.7143 13.5756 11.2899 13 12 13C12.7101 13 13.2857 13.5756 13.2857 14.2857V15.5714H10.7143V14.2857Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
