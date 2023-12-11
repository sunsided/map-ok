//! # MapOk
//!
//! This crate provides the [`MapOk`] trait that allows mapping [`Ok`] variants in an iterator to a different type.
//!
//! # Examples
//!
//! ```
//! use std::num::ParseIntError;
//! use std::str::FromStr;
//! use map_ok::MapOk;
//!
//! struct Person {
//!     age: u8,
//! }
//!
//! impl Person {
//!     fn new(age: u8) -> Self {
//!         Person { age }
//!     }
//! }
//!
//! impl FromStr for Person {
//!     type Err = ParseIntError;
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         let age = u8::from_str(s)?;
//!         Ok(Person::new(age))
//!     }
//! }
//!
//! let input = vec!["10", "20", "x", "30"];
//! let mut iterator = input.into_iter()
//!     .map(Person::from_str)
//!     .map_ok(|p| p.age);
//!
//! assert_eq!(iterator.next(), Some(Ok(10)));
//! assert_eq!(iterator.next(), Some(Ok(20)));
//! assert!(iterator.next().unwrap().is_err());
//! assert_eq!(iterator.next(), Some(Ok(30)));
//! assert_eq!(iterator.next(), None);
//! ```

mod box_ok;
mod map_ok;

pub use box_ok::{BoxOk, BoxingFn};
pub use map_ok::{MapOk, MapOkIter};

/// Commonly used imports.
pub mod prelude {
    pub use crate::{BoxOk, BoxingFn};
    pub use crate::{MapOk, MapOkIter};
}
