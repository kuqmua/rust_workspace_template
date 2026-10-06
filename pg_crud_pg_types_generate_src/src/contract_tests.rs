#[cfg(test)]
mod tests {
    #[test]
    fn test_catalog_kind_tokens_append_one_identifier_for_every_variant() {
        assert!(<crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter().all(|pg_type_catalog_kind| {
            let mut token_stream = quote::quote! { prefix };
            quote::ToTokens::to_tokens(&pg_type_catalog_kind, &mut token_stream);
            let mut emitted = token_stream.into_iter();
            matches!(emitted.next(), Some(proc_macro2::TokenTree::Ident(identifier)) if identifier == stringify!(prefix))
                && matches!(emitted.next(), Some(proc_macro2::TokenTree::Ident(identifier)) if identifier == pg_type_catalog_kind.to_string())
                && emitted.next().is_none()
        }));
    }

    #[test]
    fn test_range_catalog_conversions_preserve_all_base_types_and_reject_other_kinds() {
        let supported = [
            crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI32AsInt4Range,
            crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI64AsInt8Range,
            crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
            crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
            crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
        ];
        assert!([
            (crate::range::Range::I32AsInt4, crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsInt4),
            (crate::range::Range::I64AsInt8, crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsInt8),
            (crate::range::Range::SqlxTypesChronoNaiveDateAsDate, crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateAsDate),
            (crate::range::Range::SqlxTypesChronoNaiveDateTimeAsTimestamp, crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateTimeAsTimestamp),
            (crate::range::Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz, crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
        ].into_iter().zip(supported).all(|((range, expected), pg_type_catalog_kind)| {
            crate::pg_type_catalog_kind::PgTypeCatalogKind::from(&range) == expected
                && crate::range::Range::try_from(&pg_type_catalog_kind).is_ok_and(|decoded| {
                    crate::pg_type_catalog_kind::PgTypeCatalogKind::from(&decoded) == expected
                })
        }));
        assert!(
            <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter()
                .filter(|pg_type_catalog_kind| !supported.contains(pg_type_catalog_kind))
                .all(|pg_type_catalog_kind| matches!(
                    crate::range::Range::try_from(&pg_type_catalog_kind),
                    Err(())
                ))
        );
    }

    #[test]
    fn test_all_emitter_projections_read_one_pg_type_spec() {
        let spec = crate::pg_type_spec::PgTypeSpec::new(
            true,
            false,
            7u8,
            constants_str::PG_CRUD_PG_INT4,
            32u8,
        );
        assert_eq!(
            crate::pg_name::pg_name(&spec),
            constants_str::PG_CRUD_PG_INT4
        );
        assert_eq!(*spec.get_filter_kind(), 7u8);
        assert_eq!(crate::rust_type_wire_kind::rust_type_wire_kind(&spec), 32u8);
        assert_eq!(crate::schema_wire_kind::schema_wire_kind(&spec), 32u8);
        assert_eq!(*spec.get_wire_kind(), 32u8);
        assert!(crate::pg_type_can_be_nullable::pg_type_can_be_nullable(
            &spec
        ));
        assert!(!*spec.get_can_be_primary_key());
    }
}
