#[cfg(test)]
mod tests {
    #[test]
    fn test_catalog_initialization_origin_preserves_primary_and_nullable_capabilities() {
        assert!(
            <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter()
                .all(|kind| {
                    let name = kind.to_string();
                    let initialized_by_pg = name.ends_with(stringify!(InitializationByPg));
                    let spec = kind.spec();
                    matches!(
                        spec.get_can_be_primary_key(),
                        crate::can_be_primary_key::CanBePrimaryKey::True
                    ) == initialized_by_pg
                        && matches!(
                            spec.get_can_be_nullable(),
                            crate::can_be_nullable::CanBeNullable::True
                        ) != initialized_by_pg
                        && matches!(
                            kind.pg_type_can_be_nullable(),
                            crate::can_be_nullable::CanBeNullable::True
                        ) != initialized_by_pg
                })
        );
    }

    #[test]
    fn test_catalog_filter_categories_follow_scalar_and_range_contracts() {
        assert!(
            <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter()
                .all(|kind| {
                    let name = kind.to_string();
                    let Some((rust_name, pg_name)) = name.rsplit_once(stringify!(As)) else {
                        return false;
                    };
                    let spec = kind.spec();
                    let filter = spec.get_filter_kind();
                    if [
                        stringify!(I16),
                        stringify!(I32),
                        stringify!(I64),
                        stringify!(F32),
                        stringify!(F64),
                    ]
                    .contains(&rust_name)
                    {
                        matches!(filter, crate::filter_kind::FilterKind::Number)
                    } else if pg_name.ends_with(stringify!(Range)) {
                        matches!(filter, crate::filter_kind::FilterKind::Range)
                    } else {
                        match pg_name {
                            stringify!(Bool) => {
                                matches!(filter, crate::filter_kind::FilterKind::Bool)
                            }
                            stringify!(Bytea) => {
                                matches!(filter, crate::filter_kind::FilterKind::Bytes)
                            }
                            stringify!(Date) => {
                                matches!(filter, crate::filter_kind::FilterKind::Date)
                            }
                            stringify!(Inet) | stringify!(Interval) => {
                                matches!(filter, crate::filter_kind::FilterKind::IntervalOrInet)
                            }
                            stringify!(MacAddr) => {
                                matches!(filter, crate::filter_kind::FilterKind::Mac)
                            }
                            stringify!(Money) => {
                                matches!(filter, crate::filter_kind::FilterKind::Money)
                            }
                            stringify!(Text) => {
                                matches!(filter, crate::filter_kind::FilterKind::String)
                            }
                            stringify!(Time) => {
                                matches!(filter, crate::filter_kind::FilterKind::Time)
                            }
                            stringify!(Timestamp) => {
                                matches!(filter, crate::filter_kind::FilterKind::Timestamp)
                            }
                            stringify!(TimestampTz) => {
                                matches!(filter, crate::filter_kind::FilterKind::TimestampTz)
                            }
                            stringify!(UuidV4InitializationByPg)
                            | stringify!(UuidInitializationByClient) => {
                                matches!(filter, crate::filter_kind::FilterKind::Uuid)
                            }
                            _ => false,
                        }
                    }
                })
        );
    }

    #[test]
    fn test_money_spec_preserves_signed_integer_storage_and_wire_projections() {
        let spec = crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgMoneyAsMoney.spec();
        assert!(matches!(
            spec.get_wire_kind(),
            crate::wire_kind::WireKind::Int64
        ));
        assert!(matches!(
            crate::rust_type_wire_kind::rust_type_wire_kind(&spec),
            crate::wire_kind::WireKind::Int64
        ));
        assert!(matches!(
            crate::schema_wire_kind::schema_wire_kind(&spec),
            crate::wire_kind::WireKind::Int64
        ));
    }

    #[test]
    fn test_catalog_names_preserve_encoded_rust_prefix_and_pg_suffix() {
        assert!(
            <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter()
                .all(|kind| {
                    let name = kind.to_string();
                    let Some((rust_name, pg_name)) = name.rsplit_once(stringify!(As)) else {
                        return false;
                    };
                    crate::rust_type_name::RustTypeName::from(&kind).to_string() == rust_name
                        && crate::pg_type_name::PgTypeName::from(&kind).to_string() == pg_name
                })
        );
    }

    #[test]
    fn test_catalog_sql_names_and_literal_tokens_preserve_canonical_spellings() {
        assert!(
            <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter()
                .all(|kind| {
                    let name = kind.to_string();
                    let Some((_, pg_name)) = name.rsplit_once(stringify!(As)) else {
                        return false;
                    };
                    let expected = if pg_name == stringify!(TimestampRange) {
                        constants_str::PG_CRUD_PG_TSRANGE.to_owned()
                    } else if pg_name == stringify!(TimestampTzRange) {
                        constants_str::PG_CRUD_PG_TSTZRANGE.to_owned()
                    } else {
                        pg_name
                            .trim_end_matches(stringify!(InitializationByPg))
                            .trim_end_matches(stringify!(InitializationByClient))
                            .trim_end_matches(stringify!(V4))
                            .to_ascii_lowercase()
                };
                let pg_sql_name = crate::pg_name::pg_name(&kind.spec());
                let mut emitted = quote::quote! { #pg_sql_name }.into_iter();
                AsRef::<str>::as_ref(&pg_sql_name) == expected
                    && matches!(emitted.next(), Some(proc_macro2::TokenTree::Literal(literal))
                        if literal.to_string() == proc_macro2::Literal::string(&expected).to_string()
                            && emitted.next().is_none())
                })
        );
    }

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
