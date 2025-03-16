use dioxus::prelude::*;
#[component]
pub fn Html(
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
                d: "M4.17456 5.15007C4.08271 4.54492 4.55117 4 5.16324 4H18.8368C19.4488 4 19.9173 4.54493 19.8254 5.15007L18.0801 16.6489C18.03 16.9786 17.8189 17.2617 17.5172 17.4037L12.4258 19.7996C12.1561 19.9265 11.8439 19.9265 11.5742 19.7996L6.4828 17.4037C6.18107 17.2617 5.96997 16.9786 5.91993 16.6489L4.17456 5.15007Z",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 7.5H9.5V11H14.5V14.5L12.3714 15.3514C12.133 15.4468 11.867 15.4468 11.6286 15.3514L9.5 14.5",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Code(
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
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
                d: "M9 7L4 12L9 17",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M15 7L20 12L15 17",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
#[component]
pub fn Internet(
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
                d: "M4 15L20 15",
                stroke: "black",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M4 9L20 9",
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
            path {
                d: "M12 20.8182L11.2858 21.5181C11.4739 21.71 11.7313 21.8182 12 21.8182C12.2688 21.8182 12.5262 21.71 12.7143 21.5181L12 20.8182ZM12 3.18183L12.7143 2.48193C12.5262 2.28999 12.2688 2.18183 12 2.18183C11.7313 2.18183 11.4739 2.28999 11.2858 2.48193L12 3.18183ZM14.6 12C14.6 15.161 13.337 18.025 11.2858 20.1183L12.7143 21.5181C15.1169 19.0662 16.6 15.7053 16.6 12H14.6ZM11.2858 3.88173C13.337 5.97495 14.6 8.83897 14.6 12H16.6C16.6 8.29472 15.1169 4.93383 12.7143 2.48193L11.2858 3.88173ZM9.40002 12C9.40002 8.83897 10.6631 5.97495 12.7143 3.88173L11.2858 2.48193C8.88311 4.93383 7.40002 8.29472 7.40002 12H9.40002ZM12.7143 20.1183C10.6631 18.025 9.40002 15.161 9.40002 12H7.40002C7.40002 15.7053 8.88311 19.0662 11.2858 21.5181L12.7143 20.1183Z",
                fill: "black",
            }
        }
    }
}
#[component]
pub fn CodeOff(
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
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.83579 6.75L3.29289 11.2929C2.90237 11.6834 2.90237 12.3166 3.29289 12.7071L8.29289 17.7071C8.68342 18.0976 9.31658 18.0976 9.70711 17.7071C10.0976 17.3166 10.0976 16.6834 9.70711 16.2929L5.41421 12L9.25 8.16421L7.83579 6.75ZM15.8358 14.75L14.2929 16.2929C13.9024 16.6834 13.9024 17.3166 14.2929 17.7071C14.6834 18.0976 15.3166 18.0976 15.7071 17.7071L17.25 16.1642L15.8358 14.75ZM18.6642 14.75L20.7071 12.7071C21.0976 12.3166 21.0976 11.6834 20.7071 11.2929L15.7071 6.29289C15.3166 5.90237 14.6834 5.90237 14.2929 6.29289C13.9024 6.68342 13.9024 7.31658 14.2929 7.70711L18.5858 12L17.25 13.3358L18.6642 14.75Z",
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
