#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
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

    fn try_from(field: &syn::Field) -> Result<Self, Self::Error> {
        let mut supported_attrs = field.attrs.iter().filter_map(|element| {
            if element.path().segments.len() != 1 {
                return None;
            }
            let first_segment_identifier = &element.path().segments.first()?.ident;
            std::str::FromStr::from_str(&first_segment_identifier.to_string()).ok()
        });
        let optional_attr = supported_attrs.next();
        if supported_attrs.next().is_some() {
            return Err(constants_str::TWO_OR_MORE_SUPPORTED_ATTRS.to_owned());
        }
        optional_attr.map_or_else(|| Err(constants_str::OPT_ATTR_IS_NONE.to_owned()), Ok)
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
