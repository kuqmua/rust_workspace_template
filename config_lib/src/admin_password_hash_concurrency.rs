#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminPasswordHashConcurrency(super::super::ConfigNonZeroUsize);

impl super::super::TryFromStdEnvVarOk for AdminPasswordHashConcurrency {
    type Error = super::TryFromStdEnvVarOkAdminPasswordHashConcurrencyError;

    fn try_from_std_env_var_ok(v: super::super::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed =
            v.0.parse::<usize>()
                .map_err(|admin_positive_usize_parsing| Self::Error::Parse {
                    admin_positive_usize_parsing: super::AdminPositiveUsizeParsingError::from(
                        super::super::ParseIntError::from(admin_positive_usize_parsing),
                    ),
                })?;
        std::num::NonZeroUsize::new(parsed)
            .map(super::super::ConfigNonZeroUsize::from)
            .map(Self)
            .ok_or(Self::Error::IsZero)
    }
}
