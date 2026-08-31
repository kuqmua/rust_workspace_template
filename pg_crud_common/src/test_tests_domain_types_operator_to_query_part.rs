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
        ""
    );
    assert_eq!(
        crate::operator::Operator::Or
            .to_query_part(crate::add_operator::AddOperator::from(false))
            .as_ref(),
        ""
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
