use divan::Bencher;
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

macro_rules! convex_hull_case {
    ($group:ident, $other:expr) => {
        #[divan::bench_group]
        mod $group {
            use super::*;

            #[divan::bench]
            fn int_interval(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        (
                            I8CO::try_new(BASE.0, BASE.1).unwrap(),
                            I8CO::try_new($other.0, $other.1).unwrap(),
                        )
                    })
                    .bench_values(|(lhs, rhs)| lhs.convex_hull(rhs));
            }

            #[divan::bench]
            fn rust_intervals(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        (
                            Interval::new_closed_open(BASE.0, BASE.1),
                            Interval::new_closed_open($other.0, $other.1),
                        )
                    })
                    .bench_values(|(lhs, rhs)| lhs.convex_hull(rhs));
            }
        }
    };
}

convex_hull_case!(equal, (-32, 96));
convex_hull_case!(other_contained, (-16, 32));
convex_hull_case!(base_contained, (-64, 112));
convex_hull_case!(extends_left, (-64, 32));
convex_hull_case!(extends_right, (32, 112));
convex_hull_case!(disjoint_left, (-96, -64));
convex_hull_case!(disjoint_right, (112, 127));
