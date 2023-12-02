#[macro_export]
macro_rules! __count {
    () => (0);
    ($item:tt $($items:tt)*) => (1 + aoc::__count!($($items)*));
}

#[rustfmt::skip]
#[macro_export]
macro_rules! __splitn_parse {
    ($var:expr => str) => ($var);
    ($var:expr => char) => ($var[0]);
    ($var:expr => usize) => ($var.parse::<usize>().unwrap());
    ($var:expr => u128) => ($var.parse::<u128>().unwrap());
    ($var:expr => u64) => ($var.parse::<u64>().unwrap());
    ($var:expr => u32) => ($var.parse::<u32>().unwrap());
    ($var:expr => u16) => ($var.parse::<u16>().unwrap());
    ($var:expr => u8) => ($var.parse::<u8>().unwrap());
    ($var:expr => isize) => ($var.parse::<isize>().unwrap());
    ($var:expr => i128) => ($var.parse::<i128>().unwrap());
    ($var:expr => i64) => ($var.parse::<i64>().unwrap());
    ($var:expr => i32) => ($var.parse::<i32>().unwrap());
    ($var:expr => i16) => ($var.parse::<i16>().unwrap());
    ($var:expr => i8) => ($var.parse::<i8>().unwrap());
    ($var:expr => f64) => ($var.parse::<f64>().unwrap());
    ($var:expr => f32) => ($var.parse::<f32>().unwrap());
}

/// Split a string on a separator and parse the parts into a tuple with the given types.
#[macro_export]
macro_rules! splitn {
    ($input:expr, $sep:literal, $($type:tt),+ $(,)?) => {
        {
            let mut parts = $input.splitn(aoc::__count!($($type)+), $sep);
            (
                $(
                    aoc::__splitn_parse!(parts.next().unwrap() => $type)
                ),+
            )
        }
    };
}
