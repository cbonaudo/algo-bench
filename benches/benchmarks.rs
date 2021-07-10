use criterion::{black_box, criterion_group, criterion_main, Criterion};

use algo_bench::{get_common_prefix_igni_ugly, get_common_prefix_myo, get_common_prefix_sha};

macro_rules! bench_these {
    ( [ $( ($name:expr, $fn_name:ident)  ),* ], $parameters:expr, $criter:expr ) => {
        {
            $(
                $criter.bench_function($name, |b| b.iter(|| $fn_name (black_box($parameters))));
            )*

            benches
        }
    };
}

fn get_common_prefix(c: &mut Criterion) {
    bench_these!(
        [
            ("sha", get_common_prefix_sha),
            ("igni", get_common_prefix_igni_ugly),
            ("myo", get_common_prefix_myo)
        ],
        vec![
            "food",
            "fool",
            "foal",
            "foals",
            "fonnder",
            "follythatthiswordistoolong",
            "food",
            "fool",
            "foal",
            "foals",
            "fonnder",
            "follythatthiswordistoolong",
            "food",
            "fool",
            "foal",
            "foals",
            "fonnder",
            "follythatthiswordistoolong",
            "food",
            "fool",
            "foal",
            "foals",
            "fonnder",
            "follythatthiswordistoolong",
            "food",
            "fool",
            "foal",
            "foals",
            "fonnder",
            "follythatthiswordistoolong",
        ],
        c
    );

}

criterion_group!(benches, get_common_prefix);

criterion_main!(benches);
