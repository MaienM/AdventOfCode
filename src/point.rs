use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use derive_new::new;

// Implements an operator (add/sub/mul/div) for a point type, including the assign variant of the operator.
macro_rules! impl_operator {
    ($name:ident, $op:ident, $($vars:ident),+) => {
        paste::paste! {
            impl<T, R> [<$op:camel>]<$name<R>> for $name<T>
            where
                T: [<$op:camel>]<R>,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
            {
                type Output = Self;
                fn [<$op>](self, rhs: $name<R>) -> Self {
                    Self {
                        $($vars: self.$vars.[<$op>](rhs.$vars).into()),+
                    }
                }
            }
            impl<T, R> [<$op:camel Assign>]<$name<R>> for $name<T>
            where
                T: [<$op:camel>]<R> + Copy,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
            {
                fn [<$op _assign>](&mut self, rhs: $name<R>) {
                    *self = Self {
                        $($vars: self.$vars.[<$op>](rhs.$vars).into()),+
                    };
                }
            }
        }
    };
}

// Generate a point class with the given name and variables.
macro_rules! create_point {
    ($name:ident, $($vars:ident),+) => {
        #[derive(Clone, Copy, Eq, Hash, PartialEq, new)]
        pub struct $name<T = usize> {
            $(pub $vars: T),+
        }
        impl_operator!($name, add, $($vars),+);
        impl_operator!($name, sub, $($vars),+);
        impl_operator!($name, mul, $($vars),+);
        impl_operator!($name, div, $($vars),+);
    };
}

create_point!(Point2, x, y);
create_point!(Point3, x, y, z);

impl<T: Debug> Debug for Point2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Point({:?}, {:?})", self.x, self.y))
    }
}
impl<T: Debug> Debug for Point3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Point({:?}, {:?}, {:?})", self.x, self.y, self.z))
    }
}
