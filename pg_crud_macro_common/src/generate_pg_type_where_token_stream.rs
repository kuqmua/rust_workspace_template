#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use crate::domain_types::*;

pub fn generate_pg_type_where_token_stream<T>(
    attrs_token_stream: &dyn quote::ToTokens,
    variants: &[T],
    prefix: &dyn quote::ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDSchemarsJsonSchema,
    is_query_bind_mut: &IsQueryBindMut,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
where
    T: filters::PgFilter,
{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (
        AddOperatorSnakeCase,
        ColumnSnakeCase,
        IncrementSnakeCase,
        PgCrudCommonDefaultSomeOneElementCall,
        QuerySnakeCase,
        VSnakeCase,
    ) = (
        names.get_add_operator_snake_case(),
        names.get_column_snake_case(),
        names.get_increment_snake_case(),
        names.get_pg_crud_common_default_some_one_element_call(),
        names.get_query_snake_case(),
        names.get_v_snake_case(),
    );
    let identifier = naming::domain_types::parameter::SelfWhereUpperCamelCase::from_tokens(&prefix);
    let pg_type_tokens_where_token_stream = {
        let vrts_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.ucc();
            let prefix_where_self_upper_camel_case = element.prefix_where_self_upper_camel_case();
            let optional_type_token_stream: Option<macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream> =
                element.maybe_generic();
            let type_token_stream =
                optional_type_token_stream.map_or_else(proc_macro2::TokenStream::new, |v| quote::quote! {<#v>});
            quote::quote! {#element_upper_camel_case(where_filters::domain_types::#prefix_where_self_upper_camel_case #type_token_stream)}
        });
        let utoipa_schema_token_stream = match should_derive_utoipa_to_schema {
            ShouldDeriveUtoipaToSchema::False => proc_macro2::TokenStream::new(),
            ShouldDeriveUtoipaToSchema::True => {
                let schema_items_token_stream = variants.iter().map(|element| {
                    let element_upper_camel_case = element.ucc();
                    let prefix_where_self_upper_camel_case =
                        element.prefix_where_self_upper_camel_case();
                    let optional_type_token_stream = element.maybe_generic();
                    let type_token_stream = optional_type_token_stream.map_or_else(
                        proc_macro2::TokenStream::new,
                        |value| quote::quote! {<#value>},
                    );
                    quote::quote! {
                        .item(
                            utoipa::openapi::ObjectBuilder::new()
                                .property(
                                    stringify!(#element_upper_camel_case),
                                    <where_filters::domain_types::#prefix_where_self_upper_camel_case #type_token_stream as utoipa::PartialSchema>::schema(),
                                )
                                .required(stringify!(#element_upper_camel_case)),
                        )
                    }
                });
                let schema_name = identifier.to_string();
                quote::quote! {
                    impl utoipa::PartialSchema for #identifier {
                        fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                            utoipa::openapi::schema::Schema::from(
                                utoipa::openapi::OneOfBuilder::new()
                                    #(#schema_items_token_stream)*
                                    .build(),
                            )
                            .into()
                        }
                    }
                    impl utoipa::ToSchema for #identifier {
                        fn name() -> std::borrow::Cow<'static, str> {
                            std::borrow::Cow::Borrowed(#schema_name)
                        }
                    }
                }
            }
        };
        quote::quote! {
            #attrs_token_stream
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_schemars_json_schema, optimal_memory_layout::OptimalMemoryLayout)]
            pub enum #identifier {
                #(#vrts_token_stream),*
            }
            #utoipa_schema_token_stream
        }
    };
    let impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_token_stream =
        impl_pg_type_where_filter_for_identifier_token_stream(
            &quote::quote! {<'lt>},
            &identifier,
            &proc_macro2::TokenStream::new(),
            &IncrementParameterUndrscr::False,
            &ColumnParameterUndrscr::False,
            &AddOperatorUndrscr::False,
            &{
                let vrts_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.ucc();
                quote::quote! {
                    Self::#element_upper_camel_case(#VSnakeCase) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                        #VSnakeCase,
                        #IncrementSnakeCase,
                        #ColumnSnakeCase,
                        #AddOperatorSnakeCase,
                    )
                }
            });
                quote::quote! {
                    match &self {
                        #(#vrts_token_stream),*
                    }
                }
            },
            is_query_bind_mut,
            &{
                let vrts_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.ucc();
                quote::quote! {
                    Self::#element_upper_camel_case(#VSnakeCase) => pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(
                        #VSnakeCase,
                        #QuerySnakeCase
                    )
                }
            });
                quote::quote! {
                    match self {
                        #(#vrts_token_stream),*
                    }
                }
            },
            &Import::PgCrudCommon,
        );
    let impl_location_lib_to_err_string_for_pg_type_tokens_where_token_stream =
        generate_impl_to_err_string_no_generics_token_stream(
            &identifier,
            &quote::quote! {format!("{self:#?}")},
        );
    let impl_all_variants_default_some_one_element_for_pg_type_tokens_where_token_stream =
        generate_impl_pg_crud_common_all_variants_default_some_one_element_token_stream(
            &identifier,
            &{
                let vrts_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.ucc();
                quote::quote! {Self::#element_upper_camel_case(#PgCrudCommonDefaultSomeOneElementCall)}
            });
                quote::quote! {vec![#(#vrts_token_stream),*]}
            },
        );
    quote::quote! {
        #pg_type_tokens_where_token_stream
        #impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_token_stream
        #impl_location_lib_to_err_string_for_pg_type_tokens_where_token_stream
        #impl_all_variants_default_some_one_element_for_pg_type_tokens_where_token_stream
    }
    .into()
}
