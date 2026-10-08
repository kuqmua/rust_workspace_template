#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "location field attr keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum LocationFieldAttr {
    ErrorFieldToErrString,
    ErrorFieldToErrStringSerde,
    ErrorFieldLocation,
    ErrorFieldVecToErrString,
    ErrorFieldVecToErrStringSerde,
    ErrorFieldVecLocation,
    ErrorFieldHashMapKeyStringValueToErrString,
    ErrorFieldHashMapKeyStringValueToErrStringSerde,
    ErrorFieldHashMapKeyStringValueLocation,
}

impl std::str::FromStr for LocationFieldAttr {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|item| {
                crate::attr_identifier_str::AttrIdentifierStr::attribute_identifier_string(item)
                    .as_ref()
                    == str
            })
            .ok_or(())
    }
}

impl TryFrom<&syn::Field> for LocationFieldAttr {
    type Error = String;

    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        value
            .attrs
            .iter()
            .try_fold(None, |supported_attr, element| {
                if element.path().segments.len() != 1 {
                    return Ok(supported_attr);
                }
                let Some(first_segment_identifier) = element
                    .path()
                    .segments
                    .first()
                    .map(|segment| &segment.ident)
                else {
                    return Ok(supported_attr);
                };
                let Ok(location_field_attr) =
                    std::str::FromStr::from_str(&first_segment_identifier.to_string())
                else {
                    return Ok(supported_attr);
                };
                if !matches!(element.meta, syn::Meta::Path(_)) {
                    return Err(
                        constants_str::SUPPORTED_LOCATION_FIELD_ATTR_MUST_NOT_HAVE_ARGUMENTS
                            .to_owned(),
                    );
                }
                if supported_attr.is_some() {
                    return Err(constants_str::TWO_OR_MORE_SUPPORTED_ATTRS.to_owned());
                }
                Ok(Some(location_field_attr))
            })?
            .ok_or_else(|| constants_str::OPT_ATTR_IS_NONE.to_owned())
    }
}

impl crate::attr_identifier_str::AttrIdentifierStr for LocationFieldAttr {
    fn attribute_identifier_string(&self) -> crate::attr_identifier_name::AttrIdentifierName<'_> {
        crate::attr_identifier_name::AttrIdentifierName::from(match *self {
            Self::ErrorFieldToErrString => constants_str::ERROR_FIELD_TO_ERR_STRING,
            Self::ErrorFieldToErrStringSerde => constants_str::ERROR_FIELD_TO_ERR_STRING_SERDE,
            Self::ErrorFieldLocation => constants_str::ERROR_FIELD_LOCATION,
            Self::ErrorFieldVecToErrString => constants_str::ERROR_FIELD_VEC_TO_ERR_STRING,
            Self::ErrorFieldVecToErrStringSerde => {
                constants_str::ERROR_FIELD_VEC_TO_ERR_STRING_SERDE
            }
            Self::ErrorFieldVecLocation => constants_str::ERROR_FIELD_VEC_LOCATION,
            Self::ErrorFieldHashMapKeyStringValueToErrString => {
                constants_str::ERROR_FIELD_HASHMAP_KEY_STRING_VALUE_TO_ERR_STRING
            }
            Self::ErrorFieldHashMapKeyStringValueToErrStringSerde => {
                constants_str::ERROR_FIELD_HASHMAP_KEY_STRING_VALUE_TO_ERR_STRING_SERDE
            }
            Self::ErrorFieldHashMapKeyStringValueLocation => {
                constants_str::ERROR_FIELD_HASHMAP_KEY_STRING_VALUE_LOCATION
            }
        })
    }
}

impl LocationFieldAttr {
    const ALL: [Self; 9] = [
        Self::ErrorFieldToErrString,
        Self::ErrorFieldToErrStringSerde,
        Self::ErrorFieldLocation,
        Self::ErrorFieldVecToErrString,
        Self::ErrorFieldVecToErrStringSerde,
        Self::ErrorFieldVecLocation,
        Self::ErrorFieldHashMapKeyStringValueToErrString,
        Self::ErrorFieldHashMapKeyStringValueToErrStringSerde,
        Self::ErrorFieldHashMapKeyStringValueLocation,
    ];

    #[must_use]
    pub fn to_attr_view_token_stream(
        &self,
    ) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
        match format!(
            "#[{}]",
            crate::attr_identifier_str::AttrIdentifierStr::attribute_identifier_string(self)
                .as_ref()
        )
        .parse::<proc_macro2::TokenStream>()
        {
            Ok(v) => crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(v),
            Err(error) => super::macro_compile_error_tokens::macro_compile_error_tokens(
                super::compile_error_message::CompileErrorMessage::from(&error.to_string()),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_supported_location_field_attributes_reject_arguments() {
        let list_field: syn::Field = syn::parse_quote! {
            #[error_field_location(unexpected)] value: Location
        };
        let named_field: syn::Field = syn::parse_quote! {
            #[error_field_location = "unexpected"] value: Location
        };
        assert_eq!(
            super::LocationFieldAttr::try_from(&list_field)
                .err()
                .as_deref(),
            Some(constants_str::SUPPORTED_LOCATION_FIELD_ATTR_MUST_NOT_HAVE_ARGUMENTS)
        );
        assert_eq!(
            super::LocationFieldAttr::try_from(&named_field)
                .err()
                .as_deref(),
            Some(constants_str::SUPPORTED_LOCATION_FIELD_ATTR_MUST_NOT_HAVE_ARGUMENTS)
        );
    }

    #[test]
    fn test_supported_location_field_attributes_require_exactly_one_marker() {
        let valid_field: syn::Field = syn::parse_quote! {
            #[serde(skip)] #[error_field_location] value: Location
        };
        let duplicated_field: syn::Field = syn::parse_quote! {
            #[error_field_location] #[error_field_vec_location] value: Location
        };
        let missing_field: syn::Field = syn::parse_quote! {
            #[serde(skip)] value: Location
        };
        assert!(matches!(
            super::LocationFieldAttr::try_from(&valid_field),
            Ok(super::LocationFieldAttr::ErrorFieldLocation)
        ));
        assert_eq!(
            super::LocationFieldAttr::try_from(&duplicated_field)
                .err()
                .as_deref(),
            Some(constants_str::TWO_OR_MORE_SUPPORTED_ATTRS)
        );
        assert_eq!(
            super::LocationFieldAttr::try_from(&missing_field)
                .err()
                .as_deref(),
            Some(constants_str::OPT_ATTR_IS_NONE)
        );
    }
    #[test]
    fn test_location_field_catalog_preserves_names_token_views_and_parser_round_trips() {
        [
            (
                super::LocationFieldAttr::ErrorFieldToErrString,
                constants_str::ERROR_FIELD_TO_ERR_STRING,
            ),
            (
                super::LocationFieldAttr::ErrorFieldToErrStringSerde,
                constants_str::ERROR_FIELD_TO_ERR_STRING_SERDE,
            ),
            (
                super::LocationFieldAttr::ErrorFieldLocation,
                constants_str::ERROR_FIELD_LOCATION,
            ),
            (
                super::LocationFieldAttr::ErrorFieldVecToErrString,
                constants_str::ERROR_FIELD_VEC_TO_ERR_STRING,
            ),
            (
                super::LocationFieldAttr::ErrorFieldVecToErrStringSerde,
                constants_str::ERROR_FIELD_VEC_TO_ERR_STRING_SERDE,
            ),
            (
                super::LocationFieldAttr::ErrorFieldVecLocation,
                constants_str::ERROR_FIELD_VEC_LOCATION,
            ),
            (
                super::LocationFieldAttr::ErrorFieldHashMapKeyStringValueToErrString,
                constants_str::ERROR_FIELD_HASHMAP_KEY_STRING_VALUE_TO_ERR_STRING,
            ),
            (
                super::LocationFieldAttr::ErrorFieldHashMapKeyStringValueToErrStringSerde,
                constants_str::ERROR_FIELD_HASHMAP_KEY_STRING_VALUE_TO_ERR_STRING_SERDE,
            ),
            (
                super::LocationFieldAttr::ErrorFieldHashMapKeyStringValueLocation,
                constants_str::ERROR_FIELD_HASHMAP_KEY_STRING_VALUE_LOCATION,
            ),
        ]
        .into_iter()
        .fold((), |(), (mode, expected_name)| {
            let name =
                crate::attr_identifier_str::AttrIdentifierStr::attribute_identifier_string(&mode);
            assert_eq!(name.as_ref(), expected_name);
            assert!(
                expected_name
                    .parse::<super::LocationFieldAttr>()
                    .is_ok_and(
                        |parsed| std::mem::discriminant(&parsed) == std::mem::discriminant(&mode)
                    )
            );
            let tokens = mode.to_attr_view_token_stream();
            assert!(
                syn::parse::Parser::parse2(syn::Attribute::parse_outer, tokens.as_ref().clone())
                    .is_ok_and(|attributes| attributes.len() == 1usize
                        && attributes
                            .first()
                            .is_some_and(|attribute| attribute.path().is_ident(expected_name)
                                && matches!(attribute.meta, syn::Meta::Path(_))))
            );
            let field: syn::Field = syn::parse_quote!(#tokens value: ErrorValue);
            assert!(
                super::LocationFieldAttr::try_from(&field).is_ok_and(
                    |parsed| std::mem::discriminant(&parsed) == std::mem::discriminant(&mode)
                )
            );
        });
    }

    #[test]
    fn test_location_field_attributes_preserve_qualified_marker_ignoring_and_error_precedence() {
        let qualified: syn::Field =
            syn::parse_quote!(#[module::error_field_location] value: Location);
        [
            (qualified, constants_str::OPT_ATTR_IS_NONE),
            (syn::parse_quote!(#[error_field_location] #[error_field_location] value: Location), constants_str::TWO_OR_MORE_SUPPORTED_ATTRS),
            (syn::parse_quote!(#[error_field_location] #[error_field_vec_location(unexpected)] value: Location), constants_str::SUPPORTED_LOCATION_FIELD_ATTR_MUST_NOT_HAVE_ARGUMENTS),
            (syn::parse_quote!(#[error_field_location(unexpected)] #[error_field_location] #[error_field_vec_location] value: Location), constants_str::SUPPORTED_LOCATION_FIELD_ATTR_MUST_NOT_HAVE_ARGUMENTS),
            (syn::parse_quote!(#[error_field_location] #[error_field_vec_location] #[error_field_location(unexpected)] value: Location), constants_str::TWO_OR_MORE_SUPPORTED_ATTRS),
        ].into_iter().fold((), |(), (field, diagnostic)| {
            assert_eq!(super::LocationFieldAttr::try_from(&field).err().as_deref(), Some(diagnostic));
        });
        let field: syn::Field = syn::parse_quote!(#[module::error_field_vec_location(unexpected)] #[unknown(unexpected)] #[error_field_location] value: Location);
        assert!(matches!(
            super::LocationFieldAttr::try_from(&field),
            Ok(super::LocationFieldAttr::ErrorFieldLocation)
        ));
    }

    #[test]
    fn test_location_field_marker_names_reject_noncanonical_spelling() {
        let marker = constants_str::ERROR_FIELD_LOCATION;
        assert!(
            [
                constants_str::EMPTY.to_owned(),
                constants_str::X.to_owned(),
                marker.to_ascii_uppercase(),
                [' ', '\t'].into_iter().chain(marker.chars()).collect(),
                marker.chars().chain(std::iter::once('\n')).collect(),
                marker
                    .chars()
                    .chain(constants_str::PATH_SEPARATOR.chars())
                    .collect(),
                constants_str::PATH_SEPARATOR
                    .chars()
                    .chain(marker.chars())
                    .collect(),
            ]
            .into_iter()
            .all(|string| string.parse::<super::LocationFieldAttr>().is_err())
        );
    }
}
