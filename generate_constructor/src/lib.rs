#[cfg(test)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    reason = "the marker makes the integration-test-only derive dependency explicit"
)]
struct TestDependencyMarker;

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
        let identifier = &parsed_input.ident;
        let (impl_generics, type_generics, where_clause) = parsed_input.generics.split_for_impl();
        let identifiers = fields
            .iter()
            .map(|field| {
                field.ident.as_ref().ok_or_else(|| {
                    syn::Error::new_spanned(field, constants_str::CONSTRUCTOR_REQUIRES_NAMED_FIELDS)
                })
            })
            .collect::<syn::Result<Vec<&syn::Ident>>>()?;
        let types = fields.iter().map(|field| &field.ty);
        Ok(quote::quote! {
            #[automatically_derived]
            impl #impl_generics #identifier #type_generics #where_clause {
                #[must_use]
                #visibility const fn new(#(#identifiers: #types),*) -> Self {
                    Self { #(#identifiers),* }
                }
            }
        })
    })()
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}
