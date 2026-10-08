#[derive(
    Debug,
    proc_macro_getters::Getters,
    proc_macro_new::New,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct OrderBy<ColumnGeneric> {
    column: ColumnGeneric,
    order: Option<crate::order::Order>,
}

impl<ColumnGeneric: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for OrderBy<ColumnGeneric> {
    #[allow(
        unused_variables,
        reason = "the schema trait implementation preserves the type-based parameter name"
    )]
    fn compose(
        vec: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::COLUMN,
                <ColumnGeneric as utoipa::PartialSchema>::schema(),
            )
            .property(
                constants_str::ORDER,
                <crate::order::Order as utoipa::PartialSchema>::schema(),
            )
            .required(constants_str::COLUMN)
            .build()
            .into()
    }
}

impl<ColumnGeneric: utoipa::ToSchema> utoipa::ToSchema for OrderBy<ColumnGeneric> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(
            bounded_types::utoipa_schema_type_name::UtoipaSchemaTypeName::for_type::<Self>()
                .into_inner(),
        )
    }

    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        bounded_types::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(&mut *schemas)
            .register_item_schema::<ColumnGeneric>();
        bounded_types::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(schemas)
            .register_item_schema::<crate::order::Order>();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_order_by_schema_preserves_column_and_optional_order_contract() {
        let schema = <crate::order_by::OrderBy<crate::pagination_base::PaginationBase> as utoipa::PartialSchema>::schema();
        assert!(matches!(
            &schema,
            utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(_))
        ));
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema
        else {
            return;
        };
        assert_eq!(object.properties.len(), 2usize);
        assert_eq!(object.required, [constants_str::COLUMN]);
        assert!(
            object.properties.get(constants_str::COLUMN)
                == Some(
                    &<crate::pagination_base::PaginationBase as utoipa::PartialSchema>::schema()
                )
        );
        assert!(
            object.properties.get(constants_str::ORDER)
                == Some(&<crate::order::Order as utoipa::PartialSchema>::schema())
        );
    }

    #[test]
    fn test_order_by_serialization_and_deserialization_preserve_optional_order() {
        let column = crate::pagination_base::PaginationBase::default();
        assert!(
            [
                None,
                Some(crate::order::Order::Ascending),
                Some(crate::order::Order::Descending)
            ]
            .into_iter()
            .all(|order| {
                let order_by = crate::order_by::OrderBy::new(column, order);
                assert_eq!(order_by.get_column(), &column);
                assert_eq!(order_by.get_order(), order.as_ref());
                let expected = serde_json::json!({
                    constants_str::COLUMN: column,
                    constants_str::ORDER: order,
                });
                assert_eq!(
                    serde_json::to_value(&order_by).ok().as_ref(),
                    Some(&expected)
                );
                serde_json::from_value::<
                    crate::order_by::OrderBy<crate::pagination_base::PaginationBase>,
                >(expected)
                .is_ok_and(|decoded| {
                    decoded.get_column() == &column && decoded.get_order() == order.as_ref()
                })
            })
        );
        assert!(
            serde_json::from_value::<
                crate::order_by::OrderBy<crate::pagination_base::PaginationBase>,
            >(serde_json::json!({ constants_str::COLUMN: column }))
            .is_ok_and(|decoded| decoded.get_column() == &column && decoded.get_order().is_none())
        );
        assert!(
            serde_json::from_value::<
                crate::order_by::OrderBy<crate::pagination_base::PaginationBase>,
            >(
                serde_json::json!({ constants_str::ORDER: crate::order::Order::Ascending })
            )
            .is_err_and(|error| error.is_data())
        );
        assert!(serde_json::from_value::<crate::order_by::OrderBy<crate::pagination_base::PaginationBase>>(
            serde_json::json!({ constants_str::COLUMN: column, constants_str::ORDER: constants_str::X })
        ).is_err_and(|error| error.is_data()));
    }
}
