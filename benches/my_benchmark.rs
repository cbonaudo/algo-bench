use criterion::{black_box, criterion_group, criterion_main, Criterion};

use test_project::{get_common_prefix_igni, get_common_prefix_igni_ugly, get_common_prefix_myo, get_common_prefix_sha};

fn criterion_benchmark_sha(c: &mut Criterion) {
    c.bench_function("sha", |b| b.iter(|| get_common_prefix_sha(black_box(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()))));
}

fn criterion_benchmark_igni(c: &mut Criterion) {
    c.bench_function("igni", |b| b.iter(|| get_common_prefix_igni(black_box(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()))));
}
fn criterion_benchmark_igni_ugly(c: &mut Criterion) {
    c.bench_function("igni ugly", |b| b.iter(|| get_common_prefix_igni_ugly(black_box(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()))));
}

fn criterion_benchmark_myo(c: &mut Criterion) {
    c.bench_function("myo", |b| b.iter(|| get_common_prefix_myo(black_box(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()))));
}

criterion_group!(benches, criterion_benchmark_igni, criterion_benchmark_igni_ugly, criterion_benchmark_myo, criterion_benchmark_sha);
criterion_main!(benches);
