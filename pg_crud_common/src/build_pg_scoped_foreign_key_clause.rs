pub fn build_pg_scoped_foreign_key_clause(
    foreign_key: &crate::pg_scoped_foreign_key::PgScopedForeignKey,
) -> Result<
    crate::query_part_fragment::QueryPartFragment,
    crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
> {
    let mut clause =
        crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText::try_from(
            String::from(constants_str::test_fixtures::FOREIGN_KEY_OPENING),
        )?;
    crate::push_identifier_list::push_identifier_list(
        &mut clause,
        foreign_key.local_columns.0.as_slice(),
    );
    clause.0.push_str(constants_str::test_fixtures::REFERENCES);
    clause
        .0
        .push_str(foreign_key.referenced_table.to_string().as_str());
    clause.0.push('(');
    crate::push_identifier_list::push_identifier_list(
        &mut clause,
        foreign_key.referenced_columns.0.as_slice(),
    );
    clause.0.push(')');
    clause.0.push_str(match foreign_key.on_delete {
        crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Cascade => {
            constants_str::test_fixtures::ON_DELETE_CASCADE
        }
        crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Restrict => {
            constants_str::test_fixtures::ON_DELETE_RESTRICT
        }
    });
    crate::query_part_fragment::QueryPartFragment::try_from(clause.0)
}

#[cfg(test)]
mod tests {
    fn scoped_foreign_key_identifier(value: &str) -> crate::sql_identifier::SqlIdentifier {
        crate::sql_identifier::SqlIdentifier::try_from(value.to_owned())
            .expect("2ec15e48 identifier invariant must hold")
    }

    #[test]
    fn scoped_foreign_key_uses_validated_composite_columns() {
        let foreign_key = crate::pg_scoped_foreign_key::PgScopedForeignKey::new(
            vec![
                scoped_foreign_key_identifier(constants_str::test_fixtures::PG_TEST_FEATURE_ID),
                scoped_foreign_key_identifier(constants_str::test_fixtures::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                scoped_foreign_key_identifier(constants_str::catalog::PUBLIC),
                scoped_foreign_key_identifier(constants_str::test_fixtures::PG_TEST_FEATURES),
            ),
            vec![
                scoped_foreign_key_identifier(constants_str::catalog::SQL_NAMES_ID),
                scoped_foreign_key_identifier(constants_str::test_fixtures::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Cascade,
        )
        .expect("21fc516e scoped_foreign_key_uses_validated_composite_columns invariant must hold");
        assert_eq!(
            crate::build_pg_scoped_foreign_key_clause::build_pg_scoped_foreign_key_clause(
                &foreign_key
            )
            .expect(
                "594452b0 scoped_foreign_key_uses_validated_composite_columns invariant must hold"
            )
            .into_inner(),
            constants_str::test_fixtures::TEST_SCOPED_FOREIGN_KEY_CLAUSE
        );
    }
}
