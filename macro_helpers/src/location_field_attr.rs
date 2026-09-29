#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "location field attr keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum LocationFieldAttr {
    EoToErrString,
    EoToErrStringSerde,
    EoLocation,
    EoVecToErrString,
    EoVecToErrStringSerde,
    EoVecLocation,
    EoHashMapKStringVToErrString,
    EoHashMapKStringVToErrStringSerde,
    EoHashMapKStringVLocation,
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
            Self::EoToErrString => constants_str::EO_TO_ERR_STRING,
            Self::EoToErrStringSerde => constants_str::EO_TO_ERR_STRING_SERDE,
            Self::EoLocation => constants_str::EO_LOCATION,
            Self::EoVecToErrString => constants_str::EO_VEC_TO_ERR_STRING,
            Self::EoVecToErrStringSerde => constants_str::EO_VEC_TO_ERR_STRING_SERDE,
            Self::EoVecLocation => constants_str::EO_VEC_LOCATION,
            Self::EoHashMapKStringVToErrString => {
                constants_str::EO_HASHMAP_K_STRING_V_TO_ERR_STRING
            }
            Self::EoHashMapKStringVToErrStringSerde => {
                constants_str::EO_HASHMAP_K_STRING_V_TO_ERR_STRING_SERDE
            }
            Self::EoHashMapKStringVLocation => constants_str::EO_HASHMAP_K_STRING_V_LOCATION,
        })
    }
}

impl LocationFieldAttr {
    const ALL: [Self; 9] = [
        Self::EoToErrString,
        Self::EoToErrStringSerde,
        Self::EoLocation,
        Self::EoVecToErrString,
        Self::EoVecToErrStringSerde,
        Self::EoVecLocation,
        Self::EoHashMapKStringVToErrString,
        Self::EoHashMapKStringVToErrStringSerde,
        Self::EoHashMapKStringVLocation,
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
            #[eo_location(unexpected)] value: Location
        };
        let named_field: syn::Field = syn::parse_quote! {
            #[eo_location = "unexpected"] value: Location
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
            #[serde(skip)] #[eo_location] value: Location
        };
        let duplicated_field: syn::Field = syn::parse_quote! {
            #[eo_location] #[eo_vec_location] value: Location
        };
        let missing_field: syn::Field = syn::parse_quote! {
            #[serde(skip)] value: Location
        };
        assert!(matches!(
            super::LocationFieldAttr::try_from(&valid_field),
            Ok(super::LocationFieldAttr::EoLocation)
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
}
