use num::{BigInt, BigUint};

/// Computes the absolute difference between two numbers.
pub trait AbsDiff<T> {
    type Output;

    /// Computes the absolute difference between self and other.
    fn abs_diff(self, other: T) -> Self::Output;
}

macro_rules! impl_native {
    ($type:ty, $result:ty) => {
        impl AbsDiff<$type> for $type {
            type Output = $result;
            fn abs_diff(self, other: $type) -> Self::Output {
                <$type>::abs_diff(self, other)
            }
        }
    };
}
impl_native!(u8, u8);
impl_native!(u16, u16);
impl_native!(u32, u32);
impl_native!(u64, u64);
impl_native!(u128, u128);
impl_native!(usize, usize);
impl_native!(i8, u8);
impl_native!(i16, u16);
impl_native!(i32, u32);
impl_native!(i64, u64);
impl_native!(i128, u128);
impl_native!(isize, usize);

impl AbsDiff<BigUint> for BigUint {
    type Output = BigUint;
    fn abs_diff(self, other: BigUint) -> Self::Output {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}
impl AbsDiff<BigInt> for BigInt {
    type Output = BigUint;
    fn abs_diff(self, other: BigInt) -> Self::Output {
        if self > other {
            (self - other).to_biguint().unwrap()
        } else {
            (other - self).to_biguint().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use num::FromPrimitive;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn abs_diff_native() {
        assert_eq!(AbsDiff::abs_diff(1u8, 10), 9);
        assert_eq!(AbsDiff::abs_diff(10u8, 1), 9);
        assert_eq!(AbsDiff::abs_diff(10i8, -1), 11);
    }

    #[test]
    fn abs_diff_biguint() {
        assert_eq!(
            AbsDiff::abs_diff(BigUint::from_u8(10).unwrap(), BigUint::from_u8(1).unwrap()),
            BigUint::from_u8(9).unwrap()
        );
        assert_eq!(
            AbsDiff::abs_diff(BigUint::from_u8(1).unwrap(), BigUint::from_u8(10).unwrap()),
            BigUint::from_u8(9).unwrap()
        );
    }

    #[test]
    fn abs_diff_bigint() {
        assert_eq!(
            AbsDiff::abs_diff(BigInt::from_i8(10).unwrap(), BigInt::from_i8(-1).unwrap()),
            BigUint::from_u8(11).unwrap()
        );
        assert_eq!(
            AbsDiff::abs_diff(BigInt::from_i8(-1).unwrap(), BigInt::from_i8(10).unwrap()),
            BigUint::from_u8(11).unwrap()
        );
    }
}
