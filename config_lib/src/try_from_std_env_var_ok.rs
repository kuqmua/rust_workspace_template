use crate::StdEnvVarOk;

pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error>;
}
