use bdk::prelude::*;
use common::homepage::MemberRole;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub name: Signal<String>,
    pub image: Signal<String>,
    pub role: Signal<MemberRole>,
    pub email: Signal<String>,
    pub web: Signal<String>,
    pub linkedin: Signal<String>,
    pub github: Signal<String>,
    pub description: Signal<String>,
    pub selected_role: Signal<usize>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            name: use_signal(|| String::new()),
            image: use_signal(|| String::new()),
            role: use_signal(|| MemberRole::Developer),
            email: use_signal(|| String::new()),
            web: use_signal(|| String::new()),
            linkedin: use_signal(|| String::new()),
            github: use_signal(|| String::new()),
            description: use_signal(|| String::new()),
            selected_role: use_signal(|| 0),
            nav: use_navigator(),
        };

        Ok(ctrl)
    }

    pub fn set_member_role(&mut self, i: usize) {
        self.selected_role.set(i);
        self.role.set(MemberRole::VARIANTS[i]);
    }

    pub async fn submit(&self) {
        match common::homepage::Member::get_client(crate::config::get().main_api_endpoint)
            .create(
                self.name(),
                self.image(),
                self.role(),
                self.email(),
                if self.web().is_empty() {
                    None
                } else {
                    Some(self.web())
                },
                if self.linkedin().is_empty() {
                    None
                } else {
                    Some(self.linkedin())
                },
                if self.github().is_empty() {
                    None
                } else {
                    Some(self.github())
                },
                self.description(),
            )
            .await
        {
            Ok(_) => {
                self.nav.go_back();
            }
            Err(e) => {
                btracing::error!("Error creating member: {:?}", e);
            }
        }
    }
}
