use bdk::prelude::*;

#[component]
pub fn Pagination(total_count: i64, page_size: usize, onpage: EventHandler<usize>) -> Element {
    let total_page = (total_count as f64 / page_size as f64).ceil() as usize;
    let mut current_page = use_signal(|| 1);

    rsx! {
        div { class: "flex flex-row justify-center items-center gap-2",
            for i in 1..total_page + 1 {
                button {
                    class: "py-3 px-10 border aria-selected:bg-primary aria-[selected=false]:cursor-pointer",
                    "aria-selected": current_page() == i,
                    onclick: move |_| {
                        current_page.set(i);
                        onpage(i);
                    },
                    "{i}"
                }
            }
        }
    }
}
