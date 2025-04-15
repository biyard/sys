use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/ratel/politicians")] PoliticiansPage { lang: Language },

            #[nest("/homepage")]
                #[route("/contacts")] ContactsPage { lang: Language },
                #[route("/members")] MembersPage { lang: Language },
                #[route("/news")] NewsPage { lang: Language },
                #[route("/updates")] UpdatesPage { lang: Language },
            #[end_nest]

        #[end_layout]

        #[route("/users")] UsersPage { lang: Language },
    #[end_nest]


    #[redirect("/", || Route::UsersPage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
