//! The puzzle lib prelude, with the most commonly used elements.

pub use itertools::Itertools as _;
pub use num::traits::{
    CheckedAdd as _, CheckedDiv as _, CheckedMul as _, CheckedSub as _, Num as _, One as _,
    SaturatingAdd as _, SaturatingMul as _, SaturatingSub as _, WrappingAdd as _, WrappingMul as _,
    WrappingSub as _,
};
pub use rayon::prelude::*;
pub use tap::prelude::*;

#[doc(inline)]
pub use crate::parser::parse;
pub use crate::{
    ext::*,
    grid::{PointBoundaries, PointCollection, PointDataCollection, PointOnlyCollection},
    point::PointRange,
};
