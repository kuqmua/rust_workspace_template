pub trait ToErrString {
    fn to_err_string(&self) -> crate::domain_types::ErrorText;
}

impl<T> ToErrString for &T
where
    T: ToErrString + ?Sized,
{
    fn to_err_string(&self) -> crate::domain_types::ErrorText {
        (*self).to_err_string()
    }
}

impl<T> ToErrString for Option<T>
where
    T: std::fmt::Debug,
{
    fn to_err_string(&self) -> crate::domain_types::ErrorText {
        super::debug_to_string(self)
    }
}

impl<T, E> ToErrString for Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn to_err_string(&self) -> crate::domain_types::ErrorText {
        super::debug_to_string(self)
    }
}
