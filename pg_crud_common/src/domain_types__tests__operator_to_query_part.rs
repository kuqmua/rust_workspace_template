#[test]
fn to_query_part_includes_operator_when_requested() {
    assert_eq!(
        super::Operator::And
            .to_query_part(super::AddOperator::from(true))
            .as_ref(),
        format!("{} ", naming::domain_types::AndSnakeCase)
    );
    assert_eq!(
        super::Operator::Or
            .to_query_part(super::AddOperator::from(true))
            .as_ref(),
        format!("{} ", naming::domain_types::OrSnakeCase)
    );
}

#[test]
fn to_query_part_includes_not_suffix_for_negative_variants() {
    assert_eq!(
        super::Operator::AndNot
            .to_query_part(super::AddOperator::from(true))
            .as_ref(),
        format!(
            "{} {} ",
            naming::domain_types::AndSnakeCase,
            naming::domain_types::NotSnakeCase
        )
    );
    assert_eq!(
        super::Operator::OrNot
            .to_query_part(super::AddOperator::from(true))
            .as_ref(),
        format!(
            "{} {} ",
            naming::domain_types::OrSnakeCase,
            naming::domain_types::NotSnakeCase
        )
    );
}

#[test]
fn to_query_part_omits_operator_when_disabled_and_keeps_not_only_for_negative_variants() {
    assert_eq!(
        super::Operator::And
            .to_query_part(super::AddOperator::from(false))
            .as_ref(),
        ""
    );
    assert_eq!(
        super::Operator::Or
            .to_query_part(super::AddOperator::from(false))
            .as_ref(),
        ""
    );
    assert_eq!(
        super::Operator::AndNot
            .to_query_part(super::AddOperator::from(false))
            .as_ref(),
        format!("{} ", naming::domain_types::NotSnakeCase)
    );
    assert_eq!(
        super::Operator::OrNot
            .to_query_part(super::AddOperator::from(false))
            .as_ref(),
        format!("{} ", naming::domain_types::NotSnakeCase)
    );
}
