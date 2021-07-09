use test_project::{
    get_common_prefix_igni, get_common_prefix_igni_ugly, get_common_prefix_myo,
    get_common_prefix_sha,
};

criterion_main!(
    bench_these!(
        [get_common_prefix_igni, get_common_prefix_myo, get_common_prefix_sha],
        ["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()
    );
);
