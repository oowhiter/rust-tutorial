//! # Profile
//!
//! `profile` is an example crate for cargo build profiles.
//! This is the doc for the crate.

/// Adds one to the number given.
/// This is the doc for the following component.
///
/// # Examples
///
/// ```
/// let five = 5;
/// assert_eq!(6, profile::add_one(5));
/// ```
///
/// # Panics
///
/// # Errors
///
/// # Safety
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}
