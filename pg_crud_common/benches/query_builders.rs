#![allow(
    unused_crate_dependencies,
    reason = "the benchmark target links the library package but directly exercises only its public query API"
)]

fn identifier(value: &str) -> pg_crud_common::sql_identifier::SqlIdentifier {
    pg_crud_common::sql_identifier::SqlIdentifier::try_from(value.to_owned())
        .expect("cd596c44 identifier invariant must hold")
}
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "Criterion requires a named benchmark function, and repository policy requires iterator methods instead of for loops"
)]
fn bench_sql_select_builder(criterion: &mut criterion::Criterion) {
    [
        (
            constants_str::catalog::SQL_SELECT_BUILDER_1_COLUMN,
            constants_usize::ONE,
        ),
        (
            constants_str::catalog::SQL_SELECT_BUILDER_16_COLUMNS,
            16usize,
        ),
        (
            constants_str::catalog::SQL_SELECT_BUILDER_128_COLUMNS,
            128usize,
        ),
    ]
    .into_iter()
    .for_each(|(benchmark_name, columns_len)| {
        let columns = (constants_usize::ZERO..columns_len)
            .map(|idx| identifier(format!("column_{idx}").as_str()))
            .collect::<Vec<_>>();
        let builder = pg_crud_common::sql_select_builder::SqlSelectBuilder::new(
            pg_crud_common::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                identifier(constants_str::catalog::PUBLIC),
                identifier(constants_str::catalog::BENCHMARK_TABLE),
            ),
            pg_crud_common::sql_identifiers::SqlIdentifiers::try_from(columns)
                .expect("04b6cc99 benchmark SQL identifiers invariant must hold"),
        );
        let _criterion = criterion.bench_function(benchmark_name, |bencher| {
            bencher.iter(|| {
                let query = std::hint::black_box(&builder).build();
                let _query = std::hint::black_box(query);
            });
        });
    });
}
#[allow(
    clippy::single_call_fn,
    reason = "Criterion requires a named benchmark function consumed by its registration macro"
)]
fn bench_sql_like_pattern(criterion: &mut criterion::Criterion) {
    let input = constants_str::test_fixtures::VALUE_B62637D6.repeat(32usize);
    let _criterion = criterion.bench_function(
        constants_str::catalog::SQL_LIKE_PATTERN_RESERVED_256_BYTES,
        |bencher| {
            bencher.iter(|| {
                let pattern = pg_crud_common::build_sql_like_pattern::build_sql_like_pattern(
                    std::hint::black_box(input.as_str()).into(),
                    pg_crud_common::sql_like_match_mode::SqlLikeMatchMode::Contains,
                );
                let _pattern = std::hint::black_box(pattern);
            });
        },
    );
}
#[allow(
    clippy::single_call_fn,
    reason = "Criterion requires a named benchmark function consumed by its registration macro"
)]
fn bench_stable_read_query_plan(criterion: &mut criterion::Criterion) {
    let base = pg_crud_common::query_part_fragment::QueryPartFragment::try_from(String::from(
        constants_str::test_fixtures::TEST_READ_QUERY_BASE,
    ))
    .expect("bdca9e10 bench_stable_read_query_plan invariant must hold");
    let sort_column = identifier(constants_str::catalog::CREATED_AT);
    let tie_break_column = identifier(constants_str::catalog::SQL_NAMES_ID);
    let limit_bind = std::num::NonZeroU32::new(1u32)
        .expect("54b6f80d bench_stable_read_query_plan invariant must hold");
    let offset_bind = std::num::NonZeroU32::new(2u32)
        .expect("f05a624b bench_stable_read_query_plan invariant must hold");
    let _criterion =
        criterion.bench_function(constants_str::catalog::STABLE_READ_QUERY_PLAN, |bencher| {
            bencher.iter(|| {
                let plan =
                    pg_crud_common::build_stable_read_query_plan::build_stable_read_query_plan(
                        std::hint::black_box(base.clone()),
                        std::hint::black_box(&sort_column),
                        std::hint::black_box(&tie_break_column),
                        pg_crud_common::query_sort_order::QuerySortOrder::Descending,
                        limit_bind.into(),
                        offset_bind.into(),
                    );
                let _plan = std::hint::black_box(plan);
            });
        });
}
criterion::criterion_group!(
    query_builder_benches,
    bench_sql_select_builder,
    bench_sql_like_pattern,
    bench_stable_read_query_plan
);
criterion::criterion_main!(query_builder_benches);
