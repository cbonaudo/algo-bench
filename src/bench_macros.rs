use criterion::{black_box, criterion_group, criterion_main, Criterion};


macro_rules! bench_these {
        ( [ $( $func_name:function ),* ], $parameters:expr ) => {
        {
            $(
                fn criterion + func_name(c: &mut Criterion) {
                    c.bench_function("sha", |b| b.iter(|| func_name(black_box(parameters))));
                }
            )*
            
            criterion_group!(benches, criterion + func_name);

            benches
        }
    };
}