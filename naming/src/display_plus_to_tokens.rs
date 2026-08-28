pub trait DisplayPlusToTokens: std::fmt::Display + quote::ToTokens {}

impl<T> DisplayPlusToTokens for T where T: std::fmt::Display + quote::ToTokens {}
