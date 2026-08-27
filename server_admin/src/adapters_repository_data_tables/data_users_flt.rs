#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataUsersFlt(
    pub(super) crate::domain_types::generated_tables::StdOptionalOptionalAdminUsersWhereMany,
);
