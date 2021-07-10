use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

use algo_bench::algos::common_prefix::{
    get_common_prefix_igni, get_common_prefix_myo, get_common_prefix_sha,
};
use algo_bench::bench_these;

bench_these!(
    common_prefix,
    [
        get_common_prefix_sha,
        get_common_prefix_igni,
        get_common_prefix_myo
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
    ]
);

criterion_main!(common_prefix);
