use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[nest("/ratel")]
                #[route("/politicians")] PoliticiansPage { lang: Language },
                #[route("/presidential-candidates")] PresidentialCandidatesPage { lang: Language },
                #[route("/presidential-candidates/new")] PresidentialCandidatesNewPage { lang: Language },
                #[route("/presidential-candidates/edit/:id")] PresidentialCandidatesEditByIdPage { lang: Language, id: i64 },
            #[end_nest]

            #[nest("/homepage")]
                #[route("/contacts")] ContactsPage { lang: Language },
                #[route("/contacts/:id")] ContactsByIdPage { lang: Language, id: i64 },
                #[route("/members")] MembersPage { lang: Language },
                #[route("/members/new")] MembersNewPage { lang: Language },
                #[route("/members/edit/:id")] MembersEditPage { lang: Language, id: i64 },
                #[route("/news")] NewsPage { lang: Language },
                #[route("/news/new")] NewsNewPage { lang: Language },
                #[route("/updates")] UpdatesPage { lang: Language },
            #[end_nest]

        #[end_layout]

        #[route("/users")] UsersPage { lang: Language },
    #[end_nest]


    #[redirect("/", || Route::UsersPage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
