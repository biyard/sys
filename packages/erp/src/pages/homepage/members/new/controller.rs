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
    pub fn new(lang: Language, id: Option<i64>) -> std::result::Result<Self, RenderError> {
        let mut ctrl = Self {
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

        use_effect(move || {
            if let Some(id) = id {
                async move {
                    match common::homepage::Member::get_client(
                        crate::config::get().main_api_endpoint,
                    )
                    .get(id)
                    .await
                    {
                        Ok(member) => {
                            ctrl.name.set(member.name);
                            ctrl.image.set(member.image);
                            ctrl.role.set(member.role);
                            ctrl.email.set(member.email);
                            ctrl.web.set(member.web.unwrap_or_default());
                            ctrl.linkedin.set(member.linkedin.unwrap_or_default());
                            ctrl.github.set(member.github.unwrap_or_default());
                            ctrl.description.set(member.description);
                        }
                        Err(e) => {
                            btracing::error!("Error loading member: {:?}", e);
                        }
                    }
                };
            }
        });

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
