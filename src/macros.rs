#[macro_export]
macro_rules! bench_these {
    ( $group_name:ident, [ $( $fn_name:ident  ),* ], $parameters:expr ) => {

        paste! {
            fn [<bench_ $group_name>](c: &mut Criterion) {
                $(
                    c.bench_function(stringify!($fn_name), |b| b.iter(|| $fn_name (black_box($parameters.clone()))));
                )*
            }

            criterion_group!($group_name, [<bench_ $group_name>]);
        }
    }
}
