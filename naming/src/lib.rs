pub mod prm;

pub trait AsRefStrToScStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> AsRefStrToScStr for T
where
    T: naming_cmn::AsRefStrToScStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::AsRefStrToScStr::case(self)
    }
}

pub trait AsRefStrToScTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> AsRefStrToScTs for T
where
    T: naming_cmn::AsRefStrToScTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::AsRefStrToScTs::case_or_panic(self)
    }
}

pub trait AsRefStrToUccStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> AsRefStrToUccStr for T
where
    T: naming_cmn::AsRefStrToUccStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::AsRefStrToUccStr::case(self)
    }
}

pub trait AsRefStrToUccTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> AsRefStrToUccTs for T
where
    T: naming_cmn::AsRefStrToUccTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::AsRefStrToUccTs::case_or_panic(self)
    }
}

pub trait AsRefStrToUpperScStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> AsRefStrToUpperScStr for T
where
    T: naming_cmn::AsRefStrToUpperScStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::AsRefStrToUpperScStr::case(self)
    }
}

pub trait AsRefStrToUpperScTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> AsRefStrToUpperScTs for T
where
    T: naming_cmn::AsRefStrToUpperScTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::AsRefStrToUpperScTs::case_or_panic(self)
    }
}

pub trait DisplayToScStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> DisplayToScStr for T
where
    T: naming_cmn::DisplayToScStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::DisplayToScStr::case(self)
    }
}

pub trait DisplayToScTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> DisplayToScTs for T
where
    T: naming_cmn::DisplayToScTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::DisplayToScTs::case_or_panic(self)
    }
}

pub trait DisplayToUccStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> DisplayToUccStr for T
where
    T: naming_cmn::DisplayToUccStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::DisplayToUccStr::case(self)
    }
}

pub trait DisplayToUccTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> DisplayToUccTs for T
where
    T: naming_cmn::DisplayToUccTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::DisplayToUccTs::case_or_panic(self)
    }
}

pub trait DisplayToUpperScStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> DisplayToUpperScStr for T
where
    T: naming_cmn::DisplayToUpperScStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::DisplayToUpperScStr::case(self)
    }
}

pub trait DisplayToUpperScTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> DisplayToUpperScTs for T
where
    T: naming_cmn::DisplayToUpperScTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::DisplayToUpperScTs::case_or_panic(self)
    }
}

pub trait ToTokensToScStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> ToTokensToScStr for T
where
    T: naming_cmn::ToTokensToScStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::ToTokensToScStr::case(self)
    }
}

pub trait ToTokensToScTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> ToTokensToScTs for T
where
    T: naming_cmn::ToTokensToScTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::ToTokensToScTs::case_or_panic(self)
    }
}

pub trait ToTokensToUccStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> ToTokensToUccStr for T
where
    T: naming_cmn::ToTokensToUccStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::ToTokensToUccStr::case(self)
    }
}

pub trait ToTokensToUccTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> ToTokensToUccTs for T
where
    T: naming_cmn::ToTokensToUccTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::ToTokensToUccTs::case_or_panic(self)
    }
}

pub trait ToTokensToUpperScStr {
    #[must_use]
    fn case(&self) -> impl AsRef<str>;
}

impl<T> ToTokensToUpperScStr for T
where
    T: naming_cmn::ToTokensToUpperScStr + ?Sized,
{
    fn case(&self) -> impl AsRef<str> {
        naming_cmn::ToTokensToUpperScStr::case(self)
    }
}

pub trait ToTokensToUpperScTs {
    #[must_use]
    fn case_or_panic(&self) -> proc_macro2::TokenStream;
}

impl<T> ToTokensToUpperScTs for T
where
    T: naming_cmn::ToTokensToUpperScTs + ?Sized,
{
    fn case_or_panic(&self) -> proc_macro2::TokenStream {
        naming_cmn::ToTokensToUpperScTs::case_or_panic(self)
    }
}
