pub use automod::dir;
// pub use easy_ext::ext;
pub use envy::from_env;
pub use eyre::{bail, eyre, Result};
pub use itertools::Itertools;
// pub use monostate::MustBe;
pub use once_cell::sync::Lazy;
pub use serde::{Deserialize, Serialize};
// pub use lazy_static::lazy_static;
#[cfg(test)]
mod test {
    pub use dotenv::dotenv;
    pub use insta::assert_debug_snapshot;
}
#[cfg(test)]
pub use test::*;
