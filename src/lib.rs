//! A utility for making logs more readable by shortening and aliasing long IDs.

mod short;
pub use short::*;

mod aliased_id;
pub use aliased_id::*;

mod aliasable;
pub use aliasable::*;

pub(crate) fn bracketed(s: &str, (b1, b2): (&'static str, &'static str)) -> String {
    format!("{b1}{s}{b2}")
}
