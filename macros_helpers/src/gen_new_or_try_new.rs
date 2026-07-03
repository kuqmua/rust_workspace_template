#[must_use]
pub fn gen_new_ts<AttributeTokenStream, ParametersTokenStream, BodyTokenStream>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        fn new(#parameters) -> Self {
            #body
        }
    }
}

#[must_use]
pub fn gen_pub_new_ts<AttributeTokenStream, ParametersTokenStream, BodyTokenStream>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        pub fn new(#parameters) -> Self {
            #body
        }
    }
}

#[must_use]
pub fn gen_const_new_ts<AttributeTokenStream, ParametersTokenStream, BodyTokenStream>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        const fn new(#parameters) -> Self {
            #body
        }
    }
}

#[must_use]
pub fn gen_pub_const_new_ts<AttributeTokenStream, ParametersTokenStream, BodyTokenStream>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        pub const fn new(#parameters) -> Self {
            #body
        }
    }
}

#[must_use]
pub fn gen_impl_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            fn new(#parameters) -> Self {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_const_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            const fn new(#parameters) -> Self {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_pub_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            pub fn new(#parameters) -> Self {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_pub_const_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            pub const fn new(#parameters) -> Self {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_try_new_ts<
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        fn try_new(#parameters) -> Result<Self, #error_type> {
            #body
        }
    }
}

#[must_use]
pub fn gen_impl_try_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            fn try_new(#parameters) -> Result<Self, #error_type> {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_const_try_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            const fn try_new(#parameters) -> Result<Self, #error_type> {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_pub_try_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            pub fn try_new(#parameters) -> Result<Self, #error_type> {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_pub_const_try_new_for_ident_ts<
    IdentifierTokenStream,
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    identifier: &IdentifierTokenStream,
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #identifier {
            #attribute
            pub const fn try_new(#parameters) -> Result<Self, #error_type> {
                #body
            }
        }
    }
}
#[must_use]
pub fn gen_pub_try_new_ts<
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        pub fn try_new(#parameters) -> Result<Self, #error_type> {
            #body
        }
    }
}

#[must_use]
pub fn gen_const_try_new_ts<
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        const fn try_new(#parameters) -> Result<Self, #error_type> {
            #body
        }
    }
}

#[must_use]
pub fn gen_pub_const_try_new_ts<
    AttributeTokenStream,
    ParametersTokenStream,
    ErrorTokenStream,
    BodyTokenStream,
>(
    attribute: &AttributeTokenStream,
    parameters: &ParametersTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    AttributeTokenStream: quote::ToTokens + ?Sized,
    ParametersTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        #attribute
        pub const fn try_new(#parameters) -> Result<Self, #error_type> {
            #body
        }
    }
}
