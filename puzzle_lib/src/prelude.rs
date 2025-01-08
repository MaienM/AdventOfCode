//! The puzzle lib prelude, with the most commonly used elements.

pub use itertools::Itertools as _;
pub use rayon::prelude::*;
pub use tap::prelude::*;

pub use crate::ext::*;
#[doc(inline)]
pub use crate::parser::parse;
