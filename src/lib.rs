//! A utility for making logs more readable by shortening and aliasing long IDs.

mod renamed;
pub use renamed::*;

mod nameable;
pub use nameable::*;

mod nameables;
pub use nameables::*;

mod tests;

pub(crate) fn bracketed(s: &str, (b1, b2): (&'static str, &'static str)) -> String {
    format!("{b1}{s}{b2}")
}
