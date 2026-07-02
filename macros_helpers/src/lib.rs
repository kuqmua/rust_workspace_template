gen_derive_ts_builder::gen_derive_ts_builder!([]);

#[derive(Debug, Clone, Copy)]
pub enum DClone {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DCopy {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DDebug {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DDefault {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DEq {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DLocLibLocation {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DOrd {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DPartialEq {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DPartialOrd {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DSchemarsJsonSchema {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DSerdeDeserialize {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DSerdeSerialize {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DThiserrorError {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum DUtoipaToSchema {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum FormatWithCargofmt {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum LocFieldAttr {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum MakePub {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum ShouldWriteTsIntoFile {
    False,
    True,
}

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    Ok,
}

#[derive(Debug, Clone, Copy)]
pub enum WritePathOutcome {
    Changed,
    Unchanged,
}

#[derive(Debug, Clone, Copy)]
pub struct AttrIdentText(&'static str);

impl AsRef<str> for AttrIdentText {
    fn as_ref(&self) -> &str {
        self.0
    }
}

pub trait AttrIdentStr {
    #[must_use]
    fn attr_ident_str(&self) -> AttrIdentText;
}

#[derive(Debug, Clone, Copy)]
pub struct DTsBuilder;

impl Default for DTsBuilder {
    fn default() -> Self {
        Self
    }
}

impl DTsBuilder {
    #[must_use]
    pub fn build_enum<
        AnnotationTokenStream,
        IdentifierTokenStream,
        GenericsTokenStream,
        BodyTokenStream,
    >(
        self,
        annotation: &AnnotationTokenStream,
        identifier: &IdentifierTokenStream,
        generics: &GenericsTokenStream,
        body: &BodyTokenStream,
    ) -> proc_macro2::TokenStream
    where
        AnnotationTokenStream: quote::ToTokens + ?Sized,
        IdentifierTokenStream: quote::ToTokens + ?Sized,
        GenericsTokenStream: quote::ToTokens + ?Sized,
        BodyTokenStream: quote::ToTokens + ?Sized,
    {
        quote::quote! {
            #[derive(Debug)]
            #annotation
            enum #identifier #generics #body
        }
    }

    #[must_use]
    pub fn build_struct<
        AnnotationTokenStream,
        IdentifierTokenStream,
        GenericsTokenStream,
        BodyTokenStream,
    >(
        self,
        annotation: &AnnotationTokenStream,
        identifier: &IdentifierTokenStream,
        generics: &GenericsTokenStream,
        body: &BodyTokenStream,
    ) -> proc_macro2::TokenStream
    where
        AnnotationTokenStream: quote::ToTokens + ?Sized,
        IdentifierTokenStream: quote::ToTokens + ?Sized,
        GenericsTokenStream: quote::ToTokens + ?Sized,
        BodyTokenStream: quote::ToTokens + ?Sized,
    {
        quote::quote! {
            #[derive(Debug)]
            #annotation
            struct #identifier #generics #body
        }
    }

    #[must_use]
    pub const fn d_clone(self) -> Self {
        self
    }

    #[must_use]
    pub const fn d_copy(self) -> Self {
        self
    }

    #[must_use]
    pub const fn d_debug(self) -> Self {
        self
    }

    #[must_use]
    pub const fn d_default(self) -> Self {
        self
    }

    #[must_use]
    pub const fn d_eq(self) -> Self {
        self
    }

    #[must_use]
    pub const fn d_partial_eq(self) -> Self {
        self
    }

    #[must_use]
    pub const fn make_pub(self) -> Self {
        self
    }

    #[must_use]
    pub const fn make_pub_if(self, _condition: MakePub) -> Self {
        self
    }

    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SynField;

#[derive(Debug, Clone, Copy)]
pub struct SimpleSynPunctuated;

#[must_use]
pub fn gen_field_loc_new_ts<FileValue, LineValue, ColumnValue>(
    _file: &FileValue,
    _line: LineValue,
    _col: ColumnValue,
) -> proc_macro2::TokenStream {
    quote::quote! { loc::Loc::new(file!(), line!(), column!()) }
}

#[must_use]
pub fn gen_if_write_is_err_ts<ParametersTokenStream, BodyTokenStream>(
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        if write!(#parameters, #body).is_err() {
            return std::fmt::Result::Err(std::fmt::Error);
        }
    }
}

#[must_use]
pub fn gen_impl_dflt_ts<IdentifierTokenStream, BodyTokenStream>(
    identifier: &IdentifierTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl Default for #identifier {
            fn default() -> Self {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_display_ts<
    ImplGenericsTokenStream,
    IdentifierTokenStream,
    IdentifierGenericsTokenStream,
    BodyTokenStream,
>(
    impl_generics: &ImplGenericsTokenStream,
    identifier: &IdentifierTokenStream,
    identifier_generics: &IdentifierGenericsTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    ImplGenericsTokenStream: quote::ToTokens + ?Sized,
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    IdentifierGenericsTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #impl_generics std::fmt::Display for #identifier #identifier_generics {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_from_ts<FromTokenStream, ForTokenStream, BodyTokenStream>(
    from_type: &FromTokenStream,
    for_type: &ForTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    FromTokenStream: quote::ToTokens + ?Sized,
    ForTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl From<#from_type> for #for_type {
            fn from(value: #from_type) -> Self {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_try_from_ts<FromTokenStream, ForTokenStream, ErrorTokenStream, BodyTokenStream>(
    from_type: &FromTokenStream,
    for_type: &ForTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    FromTokenStream: quote::ToTokens + ?Sized,
    ForTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl TryFrom<#from_type> for #for_type {
            type Error = #error_type;
            fn try_from(value: #from_type) -> Result<Self, Self::Error> {
                #body
            }
        }
    }
}

#[must_use]
pub fn gen_impl_to_err_string_ts<
    ImplGenericsTokenStream,
    IdentifierTokenStream,
    IdentifierGenericsTokenStream,
    BodyTokenStream,
>(
    impl_generics: &ImplGenericsTokenStream,
    identifier: &IdentifierTokenStream,
    identifier_generics: &IdentifierGenericsTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    ImplGenericsTokenStream: quote::ToTokens + ?Sized,
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    IdentifierGenericsTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #impl_generics to_err_string::ToErrString for #identifier #identifier_generics {
            fn to_err_string(&self) -> to_err_string::ErrorString {
                #body
            }
        }
    }
}

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

#[must_use]
pub fn gen_pub_type_al_ts<IdentifierTokenStream, TypeTokenStream>(
    identifier: &IdentifierTokenStream,
    ty: &TypeTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    TypeTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! { pub type #identifier = #ty; }
}

#[must_use]
pub fn pgn_start_end_init_ts<ValueTokenStream>(value: &ValueTokenStream) -> proc_macro2::TokenStream
where
    ValueTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        let start = #value.offset;
        let end = start.saturating_add(#value.limit);
    }
}

#[must_use]
pub fn wrap_derive<ValueTokenStream>(value: &[&ValueTokenStream]) -> proc_macro2::TokenStream
where
    ValueTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! { #[derive(#(#value),*)] }
}

#[must_use]
pub const fn gen_simple_syn_punct<Value>(_value: &Value) -> SimpleSynPunctuated
where
    Value: ?Sized,
{
    SimpleSynPunctuated
}

#[must_use]
pub const fn string_syn_punct() -> SimpleSynPunctuated {
    SimpleSynPunctuated
}

#[must_use]
pub const fn get_only_one<Value>(_value: &Value) -> StatusCode {
    StatusCode::Ok
}

#[must_use]
pub fn gen_serde_version_of_named_syn_vrt<ValueTokenStream>(
    value: &ValueTokenStream,
) -> proc_macro2::TokenStream
where
    ValueTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! { #value }
}

#[must_use]
pub const fn loc_syn_field() -> SynField {
    SynField
}

#[must_use]
pub const fn try_mb_write_ts_into_file<FileNameTokenStream, TokenStreamValue>(
    _should_write: ShouldWriteTsIntoFile,
    _file_name: &FileNameTokenStream,
    _token_stream: &TokenStreamValue,
    _format_with_cargofmt: &FormatWithCargofmt,
) -> WritePathOutcome
where
    FileNameTokenStream: ?Sized,
    TokenStreamValue: ?Sized,
{
    WritePathOutcome::Unchanged
}

pub const fn mb_write_ts_into_file<FileNameTokenStream, TokenStreamValue>(
    should_write: ShouldWriteTsIntoFile,
    file_name: &FileNameTokenStream,
    token_stream: &TokenStreamValue,
    format_with_cargofmt: &FormatWithCargofmt,
) where
    FileNameTokenStream: ?Sized,
    TokenStreamValue: ?Sized,
{
    let _outcome =
        try_mb_write_ts_into_file(should_write, file_name, token_stream, format_with_cargofmt);
}
