//! Points, regions, and movement in n-dimensional space.

mod direction;
#[allow(clippy::module_inception)]
mod point;
mod range;

pub use direction::*;
pub use point::*;
pub use range::*;
