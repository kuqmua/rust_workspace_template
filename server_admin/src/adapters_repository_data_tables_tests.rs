fn filter_query(
    field: &str,
    operation: frontend_contract::filter_operation::FilterOperation,
    filter_value: Option<&str>,
    end: Option<&str>,
) -> server_admin_contract::admin_data_table_query::AdminDataTableQuery {
    server_admin_contract::admin_data_table_query::AdminDataTableQuery::new(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::admin_filter_field::AdminFilterField::try_from(
                    field.to_owned(),
                )
                .expect("a17498dc filter_query invariant must hold"),
            ),
            Some(operation),
            filter_value.map(|raw_filter_value| {
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                    raw_filter_value.to_owned(),
                )
                .expect("f064fcd7 filter_query invariant must hold")
            }),
            end.map(|raw_filter_end| {
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                    raw_filter_end.to_owned(),
                )
                .expect("9b563f27 filter_query invariant must hold")
            }),
        ),
        server_admin_contract::admin_table_query::AdminTableQuery::default(),
    )
}

#[test]
fn generated_table_fields_supply_client_column_metadata() {
    let columns = (|| -> Result<
        server_admin_contract::admin_data_columns::AdminDataColumns,
        crate::admin_repository_error::AdminRepositoryError,
    > {
        let table = server_admin_contract::admin_data_table::AdminDataTable::Users;
        let column_names = table.spec().columns();
        let generated_fields =
            crate::admin_generated_table::AdminGeneratedTable::for_data_table(table)
                .map(crate::admin_generated_table::AdminGeneratedTable::field_contracts);
        let columns = column_names
            .get()
            .split(',')
            .map(|raw_name| {
                let generated_field = generated_fields.as_ref().and_then(|fields| {
                    AsRef::<[frontend_contract::field_contract::FieldContract]>::as_ref(fields)
                        .iter()
                        .find(|field| field.name().as_ref() == raw_name)
                });
                let label_text = generated_field.map_or_else(
                    || raw_name.to_owned(),
                    |field| field.label().as_ref().to_owned(),
                );
                let input_kind = generated_field.map_or(frontend_contract::input_kind::InputKind::Text, |field| {
                    field.type_contract().input_kind()
                });
                let raw_filters = generated_field.map_or_else(Vec::new, |field| {
                    field
                        .filters()
                        .iter()
                        .copied()
                        .map(server_admin_contract::admin_data_filter::AdminDataFilter::from)
                        .collect::<Vec<_>>()
                });
                let filters =
                    server_admin_contract::admin_data_filters::AdminDataFilters::try_from(raw_filters)
                        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                let label = server_admin_contract::admin_text::AdminText::try_from(label_text)
                    .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                let name =
                    server_admin_contract::admin_text::AdminText::try_from(raw_name.to_owned())
                        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
                Ok(server_admin_contract::admin_data_column::AdminDataColumn::new(
                    filters, input_kind, label, name,
                ))
            })
            .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
        server_admin_contract::admin_data_columns::AdminDataColumns::try_from(columns)
            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
    })()
    .expect("f3c897af generated_table_fields_supply_client_column_metadata invariant must hold");
    let id = columns
        .as_slice()
        .iter()
        .find(|column| column.name().as_ref() == constants_str::SQL_NAMES_ID);
    assert!(id.is_some_and(|column| {
        column.input_kind() == frontend_contract::input_kind::InputKind::Number
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
            .map(server_admin_contract::admin_data_filter::AdminDataFilter::operation)
            .collect::<Vec<_>>(),
        vec![
            frontend_contract::filter_operation::FilterOperation::Eq,
            frontend_contract::filter_operation::FilterOperation::Regex,
        ]
    );
}

#[test]
fn generated_where_filter_builds_typed_table_predicate() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::VALUE_2BD806C9),
        None,
    );
    let filter = crate::data_filter::data_filter(
        server_admin_contract::admin_data_table::AdminDataTable::Users,
        query.filter(),
    )
    .expect("4e779df0 generated_where_filter_builds_typed_table_predicate invariant must hold")
    .expect("d9c8cf39 generated_where_filter_builds_typed_table_predicate invariant must hold");
    let mut increment =
        pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);

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
        frontend_contract::filter_operation::FilterOperation::Between,
        Some(constants_str::VALUE_2BD806C9),
        Some(constants_str::VALUE_81B637D8),
    );

    assert!(
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .is_err()
    );
}

#[test]
fn empty_filter_query_omits_the_predicate() {
    let query =
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::default();

    assert!(
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            &query
        )
        .expect("fd36a6f5 empty_filter_query_omits_the_predicate invariant must hold")
        .is_none()
    );
}

#[test]
fn incomplete_filter_queries_are_rejected() {
    let field = server_admin_contract::admin_filter_field::AdminFilterField::try_from(
        constants_str::LOGIN.to_owned(),
    )
    .expect("f1832a34 incomplete_filter_queries_are_rejected invariant must hold");
    let value = server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
        String::from(constants_str::VALUE_2BD806C9),
    )
    .expect("16849a06 incomplete_filter_queries_are_rejected invariant must hold");
    let queries = [
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(field),
            None,
            None,
            None,
        ),
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            None,
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            None,
            None,
        ),
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            None,
            None,
            Some(value.clone()),
            None,
        ),
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            None,
            None,
            None,
            Some(value),
        ),
    ];

    assert!(queries.iter().all(|query| {
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query,
        )
        .is_err()
    }));
}

#[test]
fn unknown_filter_field_is_rejected() {
    let query = filter_query(
        constants_str::UNKNOWN_ALT,
        frontend_contract::filter_operation::FilterOperation::Eq,
        Some(constants_str::VALUE_2BD806C9),
        None,
    );

    assert!(
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter()
        )
        .is_err()
    );
}

#[test]
fn scalar_and_regex_filters_reject_range_end_values() {
    let operations = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::Regex,
    ];
    assert!(operations.into_iter().all(|operation| {
        let query = filter_query(constants_str::LOGIN, operation, Some("alice"), Some("bob"));
        crate::data_filter::data_filter(
            server_admin_contract::admin_data_table::AdminDataTable::Users,
            query.filter(),
        )
        .is_err()
    }));
}

#[test]
fn regex_filter_builds_a_typed_predicate() {
    let query = filter_query(
        constants_str::LOGIN,
        frontend_contract::filter_operation::FilterOperation::Regex,
        Some(constants_str::VALUE_78C40633),
        None,
    );
    let filter = crate::data_filter::data_filter(
        server_admin_contract::admin_data_table::AdminDataTable::Users,
        query.filter(),
    )
    .expect("e0b1326d regex_filter_builds_a_typed_predicate invariant must hold")
    .expect("8a4e68fb regex_filter_builds_a_typed_predicate invariant must hold");
    let mut increment =
        pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);

    let fragment = filter
        .query_part(&mut increment)
        .expect("9f5e101d regex_filter_builds_a_typed_predicate invariant must hold");

    assert!(fragment.as_ref().contains(constants_str::LOGIN));
    assert_eq!(increment.get(), 1u64);
}

#[test]
fn filtered_sql_places_pagination_after_filter_binds() {
    let fragment = pg_crud_common::query_part_fragment::QueryPartFragment::try_from(String::from(
        constants_str::VALUE_F7A09FE1,
    ))
    .expect("45d292b8 filtered_sql_places_pagination_after_filter_binds invariant must hold");

    let (base_count, base_data) =
        crate::base_sql::base_sql(server_admin_contract::admin_data_table::AdminDataTable::Users)
            .expect(
                "44c43299 filtered_sql_places_pagination_after_filter_binds invariant must hold",
            );
    let (_count, data) = (|| -> Result<
        (
            server_admin_core::std_admin_string::StdAdminString,
            server_admin_core::std_admin_string::StdAdminString,
        ),
        crate::admin_repository_error::AdminRepositoryError,
    > {
        let count_sql = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(base_count.as_ref().as_str());
        let data_sql = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(base_data.as_ref().as_str());
        let bind_count = pg_crud_common::query_part_increment::QueryPartIncrement::from(1u64);
        let mut filtered_count = count_sql.get().to_owned();
        filtered_count.push(' ');
        filtered_count.push_str(fragment.as_ref());
        let (data_prefix, ordered_suffix) = data_sql
            .get()
            .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
            .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let order = ordered_suffix
            .strip_suffix(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
            .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let limit_index = bind_count.get().saturating_add(1u64);
        let offset_index = limit_index.saturating_add(1u64);
        let mut filtered_data = data_prefix.to_owned();
        filtered_data.push(' ');
        filtered_data.push_str(fragment.as_ref());
        filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
        filtered_data.push_str(order);
        filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_PREFIX);
        filtered_data.push_str(limit_index.to_string().as_str());
        filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_OFFSET_PREFIX);
        filtered_data.push_str(offset_index.to_string().as_str());
        let count = server_admin_core::std_admin_string::StdAdminString::try_from(filtered_count)
            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let data = server_admin_core::std_admin_string::StdAdminString::try_from(filtered_data)
            .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        Ok((count, data))
    })()
    .expect("c33365ba filtered_sql_places_pagination_after_filter_binds invariant must hold");

    assert!(data.as_ref().contains("where login = $1"));
    assert!(data.as_ref().contains("LIMIT $2 OFFSET $3"));
}

#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn table_spec_generates_bounded_projection_and_count_sql_for_every_table() {
    server_admin_contract::admin_data_table::AdminDataTable::ALL
        .into_iter()
        .for_each(|table| {
            let (count, data) = crate::base_sql::base_sql(table).expect("5f714b28 table_spec_generates_bounded_projection_and_count_sql_for_every_table invariant must hold");
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
