//! A utility for making logs more readable by shortening and aliasing long IDs.

mod newtype;
mod tests;
pub use newtype::*;

mod aliased_id;
pub use aliased_id::*;

mod contains_aliases;
pub use contains_aliases::*;

pub(crate) fn bracketed(s: &str, (b1, b2): (&'static str, &'static str)) -> String {
    format!("{b1}{s}{b2}")
}
