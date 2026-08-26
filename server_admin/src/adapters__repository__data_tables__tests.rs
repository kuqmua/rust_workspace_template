fn filter_query(
    field: &str,
    operation: frontend_contract::domain_types::FilterOperation,
    filter_value: Option<&str>,
    end: Option<&str>,
) -> server_admin_contract::domain_types::AdminDataTableQuery {
    server_admin_contract::domain_types::AdminDataTableQuery::new(
        server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::domain_types::AdminFilterField::try_from(field.to_owned())
                    .expect("a17498dc filter_query invariant must hold"),
            ),
            Some(operation),
            filter_value.map(|raw_filter_value| {
                server_admin_contract::domain_types::AdminFilterValue::try_from(
                    raw_filter_value.to_owned(),
                )
                .expect("f064fcd7 filter_query invariant must hold")
            }),
            end.map(|raw_filter_end| {
                server_admin_contract::domain_types::AdminFilterValue::try_from(
                    raw_filter_end.to_owned(),
                )
                .expect("9b563f27 filter_query invariant must hold")
            }),
        ),
        server_admin_contract::domain_types::AdminTableQuery::default(),
    )
}

#[test]
fn generated_table_fields_supply_client_column_metadata() {
    let columns = super::data_columns(
        server_admin_contract::domain_types::AdminDataTable::Users,
        server_admin_contract::domain_types::AdminDataTable::Users
            .spec()
            .columns(),
    )
    .expect("f3c897af generated_table_fields_supply_client_column_metadata invariant must hold");
    let id = columns
        .as_slice()
        .iter()
        .find(|column| column.name().as_ref() == constants_str::SQL_NAMES_ID);
    assert!(id.is_some_and(|column| {
        column.input_kind() == server_admin_contract::domain_types::AdminDataInputKind::Number
    }));
    let login = columns
        .as_slice()
        .iter()
        .find(|column| column.name().as_ref() == constants_str::LOGIN)
        .expect(
            "7a340d1f generated_table_fields_supply_client_column_metadata invariant must hold",
        );
    assert_eq!(
        login
            .filters()
            .iter()
            .map(server_admin_contract::domain_types::AdminDataFilter::operation)
            .collect::<Vec<_>>(),
        vec![
            frontend_contract::domain_types::FilterOperation::Eq,
            frontend_contract::domain_types::FilterOperation::Regex,
        ]
    );
}

#[test]
fn generated_where_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::domain_types::FilterOperation::Eq,
        Some(constants_str::VALUE_2BD806C9),
        None,
    );
    let filter = super::data_filter(
        server_admin_contract::domain_types::AdminDataTable::Users,
        query.filter(),
    )
    .expect("4e779df0 generated_where_filter_builds_typed_table_predicate invariant must hold")
    .expect("d9c8cf39 generated_where_filter_builds_typed_table_predicate invariant must hold");
    let mut increment = pg_crud_common::domain_types::QueryPartIncrement::from(constants_u64::ZERO);

    let fragment = filter
        .query_part(&mut increment)
        .expect("a25fe142 generated_where_filter_builds_typed_table_predicate invariant must hold");

    assert!(fragment.as_ref().contains(constants_str::LOGIN));
    assert!(fragment.as_ref().contains("$1"));
    assert_eq!(increment.get(), 1u64);
}

#[test]
fn unsupported_field_operation_is_rejected() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::domain_types::FilterOperation::Between,
        Some(constants_str::VALUE_2BD806C9),
        Some(constants_str::VALUE_81B637D8),
    );

    assert!(
        super::data_filter(
            server_admin_contract::domain_types::AdminDataTable::Users,
            query.filter(),
        )
        .is_err()
    );
}

#[test]
fn empty_filter_query_omits_the_predicate() {
    let query = server_admin_contract::domain_types::AdminDataTableFilterQuery::default();

    assert!(
        super::data_filter(
            server_admin_contract::domain_types::AdminDataTable::Users,
            &query
        )
        .expect("fd36a6f5 empty_filter_query_omits_the_predicate invariant must hold")
        .is_none()
    );
}

#[test]
fn incomplete_filter_queries_are_rejected() {
    let field = server_admin_contract::domain_types::AdminFilterField::try_from(
        constants_str::LOGIN.to_owned(),
    )
    .expect("f1832a34 incomplete_filter_queries_are_rejected invariant must hold");
    let value = server_admin_contract::domain_types::AdminFilterValue::try_from(String::from(
        constants_str::VALUE_2BD806C9,
    ))
    .expect("16849a06 incomplete_filter_queries_are_rejected invariant must hold");
    let queries = [
        server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
            Some(field),
            None,
            None,
            None,
        ),
        server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
            None,
            Some(frontend_contract::domain_types::FilterOperation::Eq),
            None,
            None,
        ),
        server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
            None,
            None,
            Some(value.clone()),
            None,
        ),
        server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
            None,
            None,
            None,
            Some(value),
        ),
    ];

    assert!(queries.iter().all(|query| {
        super::data_filter(
            server_admin_contract::domain_types::AdminDataTable::Users,
            query,
        )
        .is_err()
    }));
}

#[test]
fn unknown_filter_field_is_rejected() {
    let query = filter_query(
        constants_str::UNKNOWN_ALT,
        frontend_contract::domain_types::FilterOperation::Eq,
        Some(constants_str::VALUE_2BD806C9),
        None,
    );

    assert!(
        super::data_filter(
            server_admin_contract::domain_types::AdminDataTable::Users,
            query.filter()
        )
        .is_err()
    );
}

#[test]
fn scalar_and_regex_filters_reject_range_end_values() {
    let operations = [
        frontend_contract::domain_types::FilterOperation::Eq,
        frontend_contract::domain_types::FilterOperation::Regex,
    ];
    assert!(operations.into_iter().all(|operation| {
        let query = filter_query(constants_str::LOGIN, operation, Some("alice"), Some("bob"));
        super::data_filter(
            server_admin_contract::domain_types::AdminDataTable::Users,
            query.filter(),
        )
        .is_err()
    }));
}

#[test]
fn regex_filter_builds_a_typed_predicate() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::domain_types::FilterOperation::Regex,
        Some(constants_str::VALUE_78C40633),
        None,
    );
    let filter = super::data_filter(
        server_admin_contract::domain_types::AdminDataTable::Users,
        query.filter(),
    )
    .expect("e0b1326d regex_filter_builds_a_typed_predicate invariant must hold")
    .expect("8a4e68fb regex_filter_builds_a_typed_predicate invariant must hold");
    let mut increment = pg_crud_common::domain_types::QueryPartIncrement::from(constants_u64::ZERO);

    let fragment = filter
        .query_part(&mut increment)
        .expect("9f5e101d regex_filter_builds_a_typed_predicate invariant must hold");

    assert!(fragment.as_ref().contains(constants_str::LOGIN));
    assert_eq!(increment.get(), 1u64);
}

#[test]
fn filtered_sql_places_pagination_after_filter_binds() {
    let fragment = pg_crud_common::domain_types::QueryPartFragment::try_from(String::from(
        constants_str::VALUE_F7A09FE1,
    ))
    .expect("45d292b8 filtered_sql_places_pagination_after_filter_binds invariant must hold");

    let (base_count, base_data) = super::base_sql(
        server_admin_contract::domain_types::AdminDataTable::Users,
    )
    .expect("44c43299 filtered_sql_places_pagination_after_filter_binds invariant must hold");
    let (_count, data) = super::filtered_sql(
        crate::domain_types::StdAdminStrRef::from(base_count.as_ref().as_str()),
        crate::domain_types::StdAdminStrRef::from(base_data.as_ref().as_str()),
        &fragment,
        pg_crud_common::domain_types::QueryPartIncrement::from(1u64),
    )
    .expect("c33365ba filtered_sql_places_pagination_after_filter_binds invariant must hold");

    assert!(data.as_ref().contains("where login = $1"));
    assert!(data.as_ref().contains("LIMIT $2 OFFSET $3"));
}

#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn table_spec_generates_bounded_projection_and_count_sql_for_every_table() {
    server_admin_contract::domain_types::AdminDataTable::ALL
        .into_iter()
        .for_each(|table| {
            let (count, data) = super::base_sql(table).expect("5f714b28 table_spec_generates_bounded_projection_and_count_sql_for_every_table invariant must hold");
            let table_name = table.to_string();
            assert!(count.as_ref().contains(table_name.as_str()));
            assert!(data.as_ref().contains(table_name.as_str()));
            table.spec().columns().get().split(',').for_each(|column| {
                assert!(data.as_ref().contains(column));
            });
            assert!(
                data.as_ref()
                    .contains(constants_str::SERVER_ADMIN_DATA_SELECT_COLUMN_SUFFIX)
            );
            assert!(
                data.as_ref()
                    .ends_with(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
            );
        });
}
