use criterion::{black_box, criterion_group, criterion_main, Criterion};

use test_project::{get_common_prefix_if, get_common_prefix_igni, get_common_prefix_myo};


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("if", |b| b.iter(|| get_common_prefix_if(black_box(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()))));
}

fn criterion_benchmark_igni(c: &mut Criterion) {
    c.bench_function("igni: myo style", |b| b.iter(|| get_common_prefix_igni(black_box(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()))));
}

fn criterion_benchmark_myo(c: &mut Criterion) {
    c.bench_function("myo", |b| b.iter(|| get_common_prefix_myo(black_box(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()))));
}

criterion_group!(benches, criterion_benchmark_igni, criterion_benchmark_myo);
criterion_main!(benches);



