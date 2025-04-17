use bdk::prelude::*;
use common::homepage::News;

use crate::route::Route;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub category: Signal<String>,
    pub title: Signal<String>,
    pub image: Signal<String>,
    pub contents: Signal<String>,
    pub link: Signal<String>,
    pub main: Signal<bool>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            category: use_signal(|| String::new()),
            title: use_signal(|| String::new()),
            image: use_signal(|| String::new()),
            contents: use_signal(|| String::new()),
            link: use_signal(|| String::new()),
            main: use_signal(|| false),
            nav: use_navigator(),
        };

        Ok(ctrl)
    }

    pub async fn submit(&self) {
        match News::get_client(crate::config::get().main_api_endpoint)
            .create(
                self.category(),
                self.title(),
                self.image(),
                self.contents(),
                self.link(),
                self.main(),
            )
            .await
        {
            Ok(_) => {
                self.nav.replace(Route::NewsPage { lang: self.lang });
            }
            Err(e) => {
                btracing::error!("Error creating news: {:?}", e);
            }
        }
    }
}
