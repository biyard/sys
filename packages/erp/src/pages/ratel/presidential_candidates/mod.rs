mod controller;
mod i18n;
mod new;
mod page;
mod edit {
    mod _id;

    pub use _id::*;
}

pub use edit::*;
pub use new::*;
pub use page::*;
