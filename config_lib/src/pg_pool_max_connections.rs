#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
)]
pub struct PgPoolMaxConnections(u32);

impl TryFrom<u32> for PgPoolMaxConnections {
    type Error =
        crate::pg_pool_max_connections_try_from_u32_error::PgPoolMaxConnectionsTryFromU32Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for PgPoolMaxConnections {
    type Error = crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError;
    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: u32 = crate::parse_from_str_with_error::parse_from_str_with_error(
            crate::std_env_var_ok_ref::StdEnvVarOkRef::from(v.as_ref()),
            |u32_parsing| Self::Error::U32Parsing {
                u32_parsing: crate::u32_parse_int_error::U32ParseIntError::from(u32_parsing),
            },
        )?;
        Self::try_from(parsed).map_err(|pg_pool_max_connections| {
            Self::Error::PgPoolMaxConnections {
                pg_pool_max_connections,
            }
        })
    }
}
