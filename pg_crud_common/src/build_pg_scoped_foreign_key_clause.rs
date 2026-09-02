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
    crate::push_identifier_list::push_identifier_list(
        &mut clause,
        pg_scoped_foreign_key
            .get_local_columns()
            .get_inner()
            .as_slice(),
    );
    clause.get_inner_mut().push_str(constants_str::REFERENCES);
    clause.get_inner_mut().push_str(
        pg_scoped_foreign_key
            .get_referenced_table()
            .to_string()
            .as_str(),
    );
    clause.get_inner_mut().push('(');
    crate::push_identifier_list::push_identifier_list(
        &mut clause,
        pg_scoped_foreign_key
            .get_referenced_columns()
            .get_inner()
            .as_slice(),
    );
    clause.get_inner_mut().push(')');
    clause
        .get_inner_mut()
        .push_str(match pg_scoped_foreign_key.get_on_delete() {
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Cascade => {
                constants_str::ON_DELETE_CASCADE
            }
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Restrict => {
                constants_str::ON_DELETE_RESTRICT
            }
        });
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
}
