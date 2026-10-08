#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
)]
enum BetweenBindingValueFixture {
    FirstFailure,
    FirstSuccess,
    SecondFailure,
    SecondSuccess,
}

impl sqlx::Type<sqlx::Postgres> for BetweenBindingValueFixture {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for BetweenBindingValueFixture {
    fn encode_by_ref(
        &self,
        pg_argument_buffer: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        match self {
            Self::FirstFailure => Err(Box::new(std::fmt::Error)),
            Self::FirstSuccess => {
                <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&1i32, pg_argument_buffer)
            }
            Self::SecondFailure => Err(Box::new(std::io::Error::from(
                std::io::ErrorKind::InvalidData,
            ))),
            Self::SecondSuccess => {
                <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&2i32, pg_argument_buffer)
            }
        }
    }
}

#[test]
fn test_between_binding_preserves_first_encoder_error_and_failure_priority() {
    assert!(
        [
            BetweenBindingValueFixture::SecondFailure,
            BetweenBindingValueFixture::SecondSuccess
        ]
        .into_iter()
        .all(|end| {
            crate::between::Between::try_new(BetweenBindingValueFixture::FirstFailure, end)
                .is_ok_and(|between| {
                    let query = pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                        sqlx::query(constants_str::TEST_READ_QUERY_BASE),
                    );
                    pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                        between, query,
                    )
                    .is_err_and(|error| {
                        std::error::Error::source(&error)
                            .and_then(std::error::Error::source)
                            .is_some_and(<dyn std::error::Error>::is::<std::fmt::Error>)
                    })
                })
        })
    );
}

#[test]
fn test_between_binding_preserves_second_encoder_error_after_first_success() {
    assert!(
        crate::between::Between::try_new(
            BetweenBindingValueFixture::FirstSuccess,
            BetweenBindingValueFixture::SecondFailure,
        )
        .is_ok_and(|between| {
            let query = pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(sqlx::query(
                constants_str::TEST_READ_QUERY_BASE,
            ));
            pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(between, query)
                .is_err_and(|error| {
                    std::error::Error::source(&error)
                        .and_then(std::error::Error::source)
                        .and_then(|source| source.downcast_ref::<std::io::Error>())
                        .is_some_and(|source| source.kind() == std::io::ErrorKind::InvalidData)
                })
        })
    );
}

#[test]
fn test_between_binding_success_preserves_query_and_exact_parameter_count() {
    assert!(
        crate::between::Between::try_new(
            BetweenBindingValueFixture::FirstSuccess,
            BetweenBindingValueFixture::SecondSuccess,
        )
        .is_ok_and(|between| {
            let query = pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(sqlx::query(
                constants_str::TEST_READ_QUERY_BASE,
            ));
            pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(between, query)
                .is_ok_and(|sqlx_postgres_query| {
                    let mut bound_query = sqlx_postgres_query.into_inner();
                    sqlx::Execute::take_arguments(&mut bound_query).is_ok_and(|arguments| {
                        arguments.is_some_and(|pg_arguments| {
                            sqlx::Arguments::len(&pg_arguments) == 2usize
                        })
                    }) && sqlx::Execute::take_arguments(&mut bound_query)
                        .is_ok_and(|arguments| arguments.is_none())
                        && sqlx::Execute::sql(bound_query).as_str()
                            == constants_str::TEST_READ_QUERY_BASE
                })
        })
    );
}
