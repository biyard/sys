pub mod components;
pub mod config;
pub mod pages;
pub mod route;

use bdk::prelude::*;
use dioxus_oauth::prelude::FirebaseProvider;
use dioxus_popup::PopupService;
use route::Route;

fn main() {
    let conf = config::get();
    dioxus_logger::init(conf.log_level).expect("failed to init logger");
    tracing::debug!("config: {:?}", conf);
    rest_api::set_message(conf.domain.to_string());

    dioxus_aws::launch(app);
}

fn app() -> Element {
    PopupService::init();

    let css = include_str!("../public/theme.css");
    let conf = config::get();

    rsx! {
        FirebaseProvider {
            api_key: conf.firebase.api_key.clone(),
            auth_domain: conf.firebase.auth_domain.clone(),
            project_id: conf.firebase.project_id.clone(),
            storage_bucket: conf.firebase.storage_bucket.clone(),
            messaging_sender_id: conf.firebase.messaging_sender_id.clone(),
            app_id: conf.firebase.app_id.clone(),
            measurement_id: conf.firebase.measurement_id.clone(),
        }
        document::Link {
            href: asset!("/public/logos/favicon-96x96.png"),
            r#type: "image/png",
            rel: "icon",
            sizes: "96x96",
        }
        document::Link {
            href: asset!("/public/logos/favicon.svg"),
            r#type: "image/svg+xml",
            rel: "icon",
        }
        document::Link { href: asset!("/public/logos/favicon.ico"), rel: "shortcut icon" }
        document::Link {
            href: asset!("/public/logos/apple-touch-icon.png"),
            rel: "apple-touch-icon",
            sizes: "180x180",
        }

        document::Link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        document::Link {
            crossorigin: "false",
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
        }
        document::Style { href: "https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Raleway:ital,wght@0,100..900;1,100..900&display=swap" }
        document::Style { href: asset!("/public/main.css") }
        document::Style { href: asset!("/public/tailwind.css") }

        document::Script { src: "https://unpkg.com/@tailwindcss/browser@4.0.12/dist/index.global.js" }
        document::Style { r#type: "text/tailwindcss", {css} }

        Router::<Route> {}
    }
}

#[cfg(feature = "server")]
mod api {
    use bdk::prelude::*;
    use server_fn::codec::{GetUrl, Json};

    #[server(endpoint = "/version", input=GetUrl, output=Json)]
    pub async fn version() -> Result<String, ServerFnError> {
        Ok(match option_env!("VERSION") {
            Some(version) => match option_env!("COMMIT") {
                Some(commit) => format!("{}-{}", version, commit),
                None => format!("{}", version),
            },
            None => match option_env!("DATE") {
                Some(date) => date.to_string(),
                None => "unknown".to_string(),
            },
        }
        .to_string())
    }
}
