pub fn build_pg_scoped_foreign_key_clause(
    foreign_key: &crate::domain_types::PgScopedForeignKey,
) -> Result<
    crate::domain_types::QueryPartFragment,
    crate::domain_types::PgCrudStringWrapperTryFromStringError,
> {
    let mut clause = crate::domain_types::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText::try_from(
        String::from(constants_str::FOREIGN_KEY_OPENING),
    )?;
    crate::domain_types::push_identifier_list::push_identifier_list(
        &mut clause,
        foreign_key.local_columns.0.as_slice(),
    );
    clause.0.push_str(constants_str::REFERENCES);
    clause
        .0
        .push_str(foreign_key.referenced_table.to_string().as_str());
    clause.0.push('(');
    crate::domain_types::push_identifier_list::push_identifier_list(
        &mut clause,
        foreign_key.referenced_columns.0.as_slice(),
    );
    clause.0.push(')');
    clause.0.push_str(match foreign_key.on_delete {
        crate::domain_types::PgScopedForeignKeyOnDelete::Cascade => {
            constants_str::ON_DELETE_CASCADE
        }
        crate::domain_types::PgScopedForeignKeyOnDelete::Restrict => {
            constants_str::ON_DELETE_RESTRICT
        }
    });
    crate::domain_types::QueryPartFragment::try_from(clause.0)
}

#[cfg(test)]
mod tests {
    fn identifier(value: &str) -> crate::domain_types::SqlIdentifier {
        crate::domain_types::SqlIdentifier::try_from(value.to_owned())
            .expect("2ec15e48 identifier invariant must hold")
    }

    #[test]
    fn scoped_foreign_key_uses_validated_composite_columns() {
        let foreign_key = crate::domain_types::PgScopedForeignKey::new(
            vec![
                identifier(constants_str::PG_TEST_FEATURE_ID),
                identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::domain_types::SqlQualifiedIdentifier::new(
                identifier(constants_str::PUBLIC),
                identifier(constants_str::PG_TEST_FEATURES),
            ),
            vec![
                identifier(constants_str::SQL_NAMES_ID),
                identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::domain_types::PgScopedForeignKeyOnDelete::Cascade,
        )
        .expect("21fc516e scoped_foreign_key_uses_validated_composite_columns invariant must hold");
        assert_eq!(
            super::build_pg_scoped_foreign_key_clause(&foreign_key)
                .expect("594452b0 scoped_foreign_key_uses_validated_composite_columns invariant must hold")
                .into_inner(),
            constants_str::TEST_SCOPED_FOREIGN_KEY_CLAUSE
        );
    }
}
