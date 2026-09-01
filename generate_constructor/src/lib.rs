#[cfg(test)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    reason = "the marker makes the integration-test-only derive dependency explicit"
)]
struct GenerateConstructorTestDependencyMarker;

#[proc_macro_derive(New, attributes(constructor))]
pub fn new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    (|| -> syn::Result<proc_macro2::TokenStream> {
        let parsed_input = syn::parse::<syn::DeriveInput>(input)?;
        let syn::Data::Struct(data) = &parsed_input.data else {
            return Err(syn::Error::new_spanned(
                &parsed_input,
                constants_str::CONSTRUCTOR_REQUIRES_STRUCT,
            ));
        };
        let fields = match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                return Err(syn::Error::new_spanned(
                    &data.fields,
                    constants_str::CONSTRUCTOR_REQUIRES_NAMED_FIELDS,
                ));
            }
        };
        let visibility = parsed_input.attrs.iter().try_fold(
            parsed_input.vis.clone(),
            |visibility, attribute| {
                if attribute
                    .path()
                    .is_ident(constants_str::CONSTRUCTOR_ATTRIBUTE)
                {
                    attribute.parse_args::<syn::Visibility>()
                } else {
                    Ok(visibility)
                }
            },
        )?;
        let struct_identifier = &parsed_input.ident;
        let (impl_generics, type_generics, where_clause) = parsed_input.generics.split_for_impl();
        let mut constructor_fields = fields
            .iter()
            .enumerate()
            .map(|(field_index, field)| {
                let field_identifier = field.ident.as_ref().ok_or_else(|| {
                    syn::Error::new_spanned(field, constants_str::CONSTRUCTOR_REQUIRES_NAMED_FIELDS)
                })?;
                let order = field.attrs.iter().try_fold(None, |found, attribute| {
                    if !attribute
                        .path()
                        .is_ident(constants_str::CONSTRUCTOR_ATTRIBUTE)
                    {
                        return Ok(found);
                    }
                    let mut attribute_order = None;
                    attribute.parse_nested_meta(|metadata| {
                        if metadata.path.is_ident(constants_str::ORDER) {
                            if attribute_order.is_some() {
                                return Err(
                                    metadata.error(constants_str::CONSTRUCTOR_DUPLICATE_ORDER)
                                );
                            }
                            attribute_order = Some(
                                metadata
                                    .value()?
                                    .parse::<syn::LitInt>()?
                                    .base10_parse::<usize>()?,
                            );
                            Ok(())
                        } else {
                            Err(metadata.error(constants_str::CONSTRUCTOR_UNSUPPORTED_ATTRIBUTE))
                        }
                    })?;
                    if found.is_some() && attribute_order.is_some() {
                        return Err(syn::Error::new_spanned(
                            attribute,
                            constants_str::CONSTRUCTOR_DUPLICATE_ORDER,
                        ));
                    }
                    Ok(found.or(attribute_order))
                })?;
                Ok((order.unwrap_or(field_index), field_identifier, &field.ty))
            })
            .collect::<syn::Result<Vec<(usize, &syn::Ident, &syn::Type)>>>()?;
        constructor_fields.sort_by_key(|(order, _, _)| *order);
        constructor_fields.iter().enumerate().try_for_each(
            |(expected_order, (actual_order, field_identifier, _))| {
                if expected_order == *actual_order {
                    Ok(())
                } else {
                    Err(syn::Error::new_spanned(
                        field_identifier,
                        constants_str::CONSTRUCTOR_ORDER_MUST_BE_UNIQUE_AND_CONTIGUOUS,
                    ))
                }
            },
        )?;
        let parameter_identifiers = constructor_fields
            .iter()
            .map(|(_, field_identifier, _)| *field_identifier)
            .collect::<Vec<&syn::Ident>>();
        let types = constructor_fields.iter().map(|(_, _, ty)| ty);
        let initialization_identifiers = constructor_fields
            .iter()
            .map(|(_, field_identifier, _)| *field_identifier)
            .collect::<Vec<&syn::Ident>>();
        Ok(quote::quote! {
            #[automatically_derived]
            impl #impl_generics #struct_identifier #type_generics #where_clause {
                #[must_use]
                #visibility const fn new(#(#parameter_identifiers: #types),*) -> Self {
                    Self { #(#initialization_identifiers),* }
                }
            }
        })
    })()
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}
