pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error>;
}
