#[test]
fn test_equality_operator_preserves_distinct_sql_fragments() {
    [
        (
            crate::eq_operator::EqOperator::Eq,
            constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR,
        ),
        (
            crate::eq_operator::EqOperator::IsNull,
            constants_str::IS_NULL,
        ),
    ]
    .into_iter()
    .fold((), |(), (eq_operator, expected)| {
        let query = eq_operator.to_query_str();
        assert_eq!(query.as_ref(), expected);
        assert_eq!(query.to_string(), expected);
    });
}

#[test]
fn test_to_query_part_includes_operator_when_requested() {
    assert_eq!(
        crate::operator::Operator::And
            .to_query_part(crate::add_operator::AddOperator::from(true))
            .as_ref(),
        format!("{} ", naming::domain_types::AndSnakeCase)
    );
    assert_eq!(
        crate::operator::Operator::Or
            .to_query_part(crate::add_operator::AddOperator::from(true))
            .as_ref(),
        format!("{} ", naming::domain_types::OrSnakeCase)
    );
}

#[test]
fn test_to_query_part_includes_not_suffix_for_negative_variants() {
    assert_eq!(
        crate::operator::Operator::AndNot
            .to_query_part(crate::add_operator::AddOperator::from(true))
            .as_ref(),
        format!(
            "{} {} ",
            naming::domain_types::AndSnakeCase,
            naming::domain_types::NotSnakeCase
        )
    );
    assert_eq!(
        crate::operator::Operator::OrNot
            .to_query_part(crate::add_operator::AddOperator::from(true))
            .as_ref(),
        format!(
            "{} {} ",
            naming::domain_types::OrSnakeCase,
            naming::domain_types::NotSnakeCase
        )
    );
}

#[test]
fn test_to_query_part_omits_operator_when_disabled_and_keeps_not_only_for_negative_variants() {
    assert_eq!(
        crate::operator::Operator::And
            .to_query_part(crate::add_operator::AddOperator::from(false))
            .as_ref(),
        constants_str::EMPTY
    );
    assert_eq!(
        crate::operator::Operator::Or
            .to_query_part(crate::add_operator::AddOperator::from(false))
            .as_ref(),
        constants_str::EMPTY
    );
    assert_eq!(
        crate::operator::Operator::AndNot
            .to_query_part(crate::add_operator::AddOperator::from(false))
            .as_ref(),
        format!("{} ", naming::domain_types::NotSnakeCase)
    );
    assert_eq!(
        crate::operator::Operator::OrNot
            .to_query_part(crate::add_operator::AddOperator::from(false))
            .as_ref(),
        format!("{} ", naming::domain_types::NotSnakeCase)
    );
}
