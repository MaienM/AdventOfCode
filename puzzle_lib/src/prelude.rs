//! The puzzle lib prelude, with the most commonly used elements.

pub use itertools::Itertools as _;
pub use rayon::prelude::*;
pub use tap::prelude::*;

#[doc(inline)]
pub use crate::parser::parse;
pub use crate::{
    ext::*,
    grid::{PointBoundaries, PointCollection, PointDataCollection, PointOnlyCollection},
    point::PointRange,
};
