#![allow(
    unused_crate_dependencies,
    reason = "the benchmark target links the library package but directly exercises only its public query API"
)]
fn identifier(value: &str) -> pg_crud_cmn::SqlIdentifier {
    pg_crud_cmn::SqlIdentifier::try_from(value.to_owned()).expect("cd596c44")
}
#[allow(
    clippy::single_call_fn,
    reason = "Criterion requires a named benchmark function consumed by its registration macro"
)]
fn bench_sql_select_builder(criterion: &mut criterion::Criterion) {
    let columns = (0usize..128usize)
        .map(|idx| identifier(format!("column_{idx}").as_str()))
        .collect::<Vec<_>>();
    let _criterion = criterion.bench_function("sql_select_builder_128_columns", |bencher| {
        bencher.iter(|| {
            let query = pg_crud_cmn::SqlSelectBuilder::new(
                pg_crud_cmn::SqlQualifiedIdentifier::new(
                    identifier("public"),
                    identifier("benchmark_table"),
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
