use aoc::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};
use num::BigUint;

macro_rules! test_bench_num {
    ($c:ident, $num:expr, $expected:expr $(,)?) => {
        test_bench_num!($c, $num, $expected, factorize());
    };
    ($c:ident, $num:expr, $expected:expr, $func:ident( $($args:expr),* $(,)? ) $(,)?) => {
        {
            let num = $num;
            let name = {
                let as_str = num.to_string();
                let as_exp = format!("~{}e{}", &as_str[0..4], as_str.len() - 4);
                if as_exp.len() < as_str.len() {
                    as_exp
                } else {
                    as_str
                }
            };
            let name = format!("{name}.{}()", stringify!($func));
            assert_eq!(num.$func($($args),*), $expected, "{name}");
            $c.bench_function(&name, move |b| {
                b.iter(|| num.$func($($args),*));
            });
        }
    };
}

fn bench_factorize(c: &mut Criterion) {
    // A prime number, which will have itself as its only factor. This is pretty much worst-case performance wise.
    test_bench_num!(c, 4999, vec![4999]);

    // A large number with lots of small factors (in fact, this number is the first 25 primes multiplied together).
    test_bench_num!(
        c,
        2_305_567_963_945_518_424_753_102_147_331_756_070u128,
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ],
    );

    // As the previous test, but now with the first 250 primes. Looks daunting but should be pretty quick still, though math on a BigUint is noticeably slower than the native uint types.
    test_bench_num!(
        c,
        BigUint::parse_bytes(b"12546824917742270487305744119826516778755973335815816996294498626669640952266483874959777349275729184252715963979668955675901701628557954828632018622727276616519716326959460200877174325330092470608154690687316590029878908267456090607985648102529891672826773309878896133255941307240398932336530415839389568489106387593665809545817402902941120947426759208129367469812827126463414761349463181819563929003131146136770433965048466875776179071589652814742160007513216476363403818694860398246587598008504971218386763346753197278191331583480438115968263529847935805028994634662316632463025658040106384252635000840166401889348941418230630291758422303205747023179473557711721130190", 10).unwrap(),
        vec![2u16, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583].into_iter().map(Into::<BigUint>::into).collect::<Vec<_>>(),
    );

    // A much smaller number made up of two large factors. Doesn't look like much compared to the previous cases, but is actually the slowest of the bunch.
    let num = 11_111_111_111_111_111u64;
    test_bench_num!(c, num, vec![2_071_723, 5_363_222_357]);

    // The same thing, but with more precomputed primes.
    let primes: Vec<_> = usize::primes(num.isqrt() as usize);
    test_bench_num!(
        c,
        num,
        vec![2_071_723, 5_363_222_357],
        factorize_with_primes(primes.iter()),
    );
}

criterion_group!(benches, bench_factorize);
criterion_main!(benches);
