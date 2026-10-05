pub fn build_pg_scoped_foreign_key_clause(
    pg_scoped_foreign_key: &crate::pg_scoped_foreign_key::PgScopedForeignKey,
) -> Result<
    crate::query_part_fragment::QueryPartFragment,
    crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
> {
    let mut clause =
        crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText::try_from(
            String::from(constants_str::FOREIGN_KEY_OPENING),
        )?;
    let map_bounded_error = |source| {
        match source {
        bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
            actual_length,
            maximum_length,
        } => crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
            len: actual_length.get(),
            max: maximum_length.get(),
        },
        bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
            actual_length,
            minimum_length,
        } => crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
            len: actual_length.get(),
            max: minimum_length.get(),
        },
    }
    };
    crate::push_identifier_list::push_identifier_list(
        &mut clause,
        pg_scoped_foreign_key
            .get_local_columns()
            .get_inner()
            .as_slice(),
    )
    .map_err(map_bounded_error)?;
    clause
        .get_inner_mut()
        .try_push_str(constants_str::REFERENCES)
        .map_err(map_bounded_error)?;
    clause
        .get_inner_mut()
        .try_push_str(
            pg_scoped_foreign_key
                .get_referenced_table()
                .to_string()
                .as_str(),
        )
        .map_err(map_bounded_error)?;
    clause
        .get_inner_mut()
        .try_push('(')
        .map_err(map_bounded_error)?;
    crate::push_identifier_list::push_identifier_list(
        &mut clause,
        pg_scoped_foreign_key
            .get_referenced_columns()
            .get_inner()
            .as_slice(),
    )
    .map_err(map_bounded_error)?;
    clause
        .get_inner_mut()
        .try_push(')')
        .map_err(map_bounded_error)?;
    clause
        .get_inner_mut()
        .try_push_str(match pg_scoped_foreign_key.get_on_delete() {
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Cascade => {
                constants_str::ON_DELETE_CASCADE
            }
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Restrict => {
                constants_str::ON_DELETE_RESTRICT
            }
        })
        .map_err(map_bounded_error)?;
    crate::query_part_fragment::QueryPartFragment::try_from(clause.into_inner())
}

#[cfg(test)]
mod tests {
    fn scoped_foreign_key_identifier(str: &str) -> crate::sql_identifier::SqlIdentifier {
        crate::sql_identifier::SqlIdentifier::try_from(str.to_owned())
            .expect(constants_str::DIAGNOSTIC_2EC15E48)
    }

    #[test]
    fn test_scoped_foreign_key_uses_validated_composite_columns() {
        let foreign_key = crate::pg_scoped_foreign_key::PgScopedForeignKey::new(
            vec![
                scoped_foreign_key_identifier(constants_str::PG_TEST_FEATURE_ID),
                scoped_foreign_key_identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                scoped_foreign_key_identifier(constants_str::PUBLIC),
                scoped_foreign_key_identifier(constants_str::PG_TEST_FEATURES),
            ),
            vec![
                scoped_foreign_key_identifier(constants_str::SQL_NAMES_ID),
                scoped_foreign_key_identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Cascade,
        )
        .expect(constants_str::DIAGNOSTIC_21FC516E);
        assert_eq!(
            crate::build_pg_scoped_foreign_key_clause::build_pg_scoped_foreign_key_clause(
                &foreign_key
            )
            .expect(constants_str::DIAGNOSTIC_594452B0)
            .into_inner(),
            constants_str::TEST_SCOPED_FOREIGN_KEY_CLAUSE
        );
    }

    #[test]
    fn test_scoped_foreign_key_restrict_rendering_preserves_composite_column_order() {
        let foreign_key_result = crate::pg_scoped_foreign_key::PgScopedForeignKey::new(
            vec![
                scoped_foreign_key_identifier(constants_str::PG_TEST_FEATURE_ID),
                scoped_foreign_key_identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                scoped_foreign_key_identifier(constants_str::PUBLIC),
                scoped_foreign_key_identifier(constants_str::PG_TEST_FEATURES),
            ),
            vec![
                scoped_foreign_key_identifier(constants_str::SQL_NAMES_ID),
                scoped_foreign_key_identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Restrict,
        );
        assert!(foreign_key_result.is_ok());
        let Ok(foreign_key) = foreign_key_result else {
            return;
        };
        let prefix_result = constants_str::TEST_SCOPED_FOREIGN_KEY_CLAUSE
            .strip_suffix(constants_str::ON_DELETE_CASCADE);
        assert!(prefix_result.is_some());
        let Some(prefix) = prefix_result else {
            return;
        };
        let expected = [prefix, constants_str::ON_DELETE_RESTRICT].concat();
        assert!(
            crate::build_pg_scoped_foreign_key_clause::build_pg_scoped_foreign_key_clause(
                &foreign_key
            )
            .is_ok_and(|clause| clause.into_inner() == expected)
        );
    }

    #[test]
    fn test_scoped_foreign_key_rendering_accepts_maximum_columns_and_identifier_lengths() {
        let maximum_columns =
            crate::maximum_scoped_foreign_key_columns::MAXIMUM_SCOPED_FOREIGN_KEY_COLUMNS;
        let columns = || {
            (0usize..maximum_columns)
                .map(|index| {
                    scoped_foreign_key_identifier(
                        format!("{}{}", constants_str::X.repeat(126usize), index).as_str(),
                    )
                })
                .collect::<Vec<_>>()
                .into()
        };
        let longest_identifier = constants_str::X.repeat(128usize);
        let foreign_key_result = crate::pg_scoped_foreign_key::PgScopedForeignKey::new(
            columns(),
            crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                scoped_foreign_key_identifier(&longest_identifier),
                scoped_foreign_key_identifier(&longest_identifier),
            ),
            columns(),
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Restrict,
        );
        assert!(foreign_key_result.is_ok());
        let Ok(foreign_key) = foreign_key_result else {
            return;
        };
        let expected_columns = foreign_key
            .get_local_columns()
            .get_inner()
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>()
            .join(constants_str::TEXT_ALT_6);
        let expected_table =
            [longest_identifier.as_str(), longest_identifier.as_str()].join(constants_str::DOT);
        let expected = [
            constants_str::FOREIGN_KEY_OPENING,
            expected_columns.as_str(),
            constants_str::REFERENCES,
            expected_table.as_str(),
            '('.to_string().as_str(),
            expected_columns.as_str(),
            ')'.to_string().as_str(),
            constants_str::ON_DELETE_RESTRICT,
        ]
        .concat();
        assert!(
            crate::build_pg_scoped_foreign_key_clause::build_pg_scoped_foreign_key_clause(
                &foreign_key
            )
            .is_ok_and(|clause| clause.into_inner() == expected)
        );
    }
}
