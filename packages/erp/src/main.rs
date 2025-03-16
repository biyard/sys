pub mod config;
pub mod pages;
pub mod route;

use bdk::prelude::*;
use dioxus_oauth::prelude::FirebaseService;
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
    let firebase = &config::get().firebase;
    let firebase = FirebaseService::new(
        firebase.api_key.clone(),
        firebase.auth_domain.clone(),
        firebase.project_id.clone(),
        firebase.storage_bucket.clone(),
        firebase.messaging_sender_id.clone(),
        firebase.app_id.clone(),
        firebase.measurement_id.clone(),
    );

    use_context_provider(move || firebase);

    let css = include_str!("../public/input.css");

    rsx! {
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
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Raleway:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }
        document::Style { href: asset!("/public/main.css") }
        document::Style { href: asset!("/public/tailwind.css") }

        document::Script { src: "https://unpkg.com/@tailwindcss/browser@4.0.12/dist/index.global.js" }
        document::Style { r#type: "text/tailwindcss", {css} }

        document::Script { r#type: "module", src: asset!("/public/dep.js"), defer: true }

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
