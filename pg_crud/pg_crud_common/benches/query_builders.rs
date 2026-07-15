#![allow(
    unused_crate_dependencies,
    reason = "the benchmark target links the library package but directly exercises only its public query API"
)]
fn identifier(value: &str) -> pg_crud_common::SqlIdentifier {
    pg_crud_common::SqlIdentifier::try_from(value.to_owned()).expect("cd596c44")
}
#[allow(
    clippy::single_call_fn,
    reason = "Criterion requires a named benchmark function consumed by its registration macro"
)]
fn bench_sql_select_builder(criterion: &mut criterion::Criterion) {
    let columns = (0usize..128usize)
        .map(|idx| identifier(format!("column_{idx}").as_str()))
        .collect::<Vec<_>>();
    let _criterion =
        criterion.bench_function(str_constants::SQL_SELECT_BUILDER_128_COLUMNS, |bencher| {
            bencher.iter(|| {
                let query = pg_crud_common::SqlSelectBuilder::new(
                    pg_crud_common::SqlQualifiedIdentifier::new(
                        identifier(str_constants::PUBLIC),
                        identifier(str_constants::BENCHMARK_TABLE),
                    ),
                    columns.clone(),
                )
                .build();
                let _query = std::hint::black_box(query);
            });
        });
}
criterion::criterion_group!(query_builder_benches, bench_sql_select_builder);
criterion::criterion_main!(query_builder_benches);
