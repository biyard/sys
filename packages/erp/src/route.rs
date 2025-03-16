use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]

            #[route("/")] HomePage { lang: Language },

        #[end_layout]

        #[route("/users")] UsersPage { lang: Language },
    #[end_nest]


    #[redirect("/", || Route::UsersPage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
