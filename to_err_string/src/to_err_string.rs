pub trait ToErrString {
    fn to_err_string(&self) -> crate::error_text::ErrorText;
}

impl<T> ToErrString for &T
where
    T: ToErrString + ?Sized,
{
    fn to_err_string(&self) -> crate::error_text::ErrorText {
        (*self).to_err_string()
    }
}

impl<T> ToErrString for Option<T>
where
    T: std::fmt::Debug,
{
    fn to_err_string(&self) -> crate::error_text::ErrorText {
        crate::debug_to_string::debug_to_string(self)
    }
}

impl<T, E> ToErrString for Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn to_err_string(&self) -> crate::error_text::ErrorText {
        crate::debug_to_string::debug_to_string(self)
    }
}
