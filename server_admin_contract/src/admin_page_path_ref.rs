#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(crate))]
pub struct AdminPagePathRef<'path_lt>(&'path_lt str);

impl<'path_lt> AdminPagePathRef<'path_lt> {
    #[must_use]
    pub fn is_navigation_section(
        self,
        admin_data_table_frontend_path: &crate::admin_data_table_frontend_path::AdminDataTableFrontendPath,
    ) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(
            self.get()
                .strip_prefix(admin_data_table_frontend_path.as_ref())
                .is_some_and(|suffix| suffix.is_empty() || suffix.starts_with('/'))
                || (admin_data_table_frontend_path
                    == &crate::admin_data_table::AdminDataTable::Rules.frontend_path()
                    && crate::admin_rule_id::AdminRuleId::from_frontend_path(self).is_some()),
        )
    }

    pub(crate) fn record_identifier(
        self,
        admin_data_table: crate::admin_data_table::AdminDataTable,
    ) -> Option<&'path_lt str> {
        self.get()
            .strip_prefix(admin_data_table.frontend_path().as_ref())
            .and_then(|value| value.strip_prefix('/'))
    }

    pub(crate) fn record_read_id(
        self,
        admin_data_table: crate::admin_data_table::AdminDataTable,
        admin_frontend_path: crate::admin_frontend_path::AdminFrontendPath,
    ) -> Option<crate::positive_non_zero_i64::PositiveNonZeroI64> {
        self.record_id(admin_data_table).or_else(|| {
            let (prefix, parameter_suffix) = admin_frontend_path.get().split_once('{')?;
            let (_parameter, suffix) = parameter_suffix.split_once('}')?;
            let value = self
                .get()
                .strip_prefix(prefix)?
                .strip_suffix(suffix)?
                .parse::<i64>()
                .ok()?;
            crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).ok()
        })
    }

    pub(crate) fn record_id(
        self,
        admin_data_table: crate::admin_data_table::AdminDataTable,
    ) -> Option<crate::positive_non_zero_i64::PositiveNonZeroI64> {
        let value = self
            .record_identifier(admin_data_table)
            .and_then(|value| value.parse::<i64>().ok())?;
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).ok()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository policy forbids for loops; iterator traversal checks every navigation fixture"
    )]
    fn test_navigation_section_matches_lists_and_details_without_matching_adjacent_sections() {
        [
            (
                crate::admin_data_table::AdminDataTable::Rules,
                crate::admin_frontend_path::AdminFrontendPath::RuleRecordRead,
                constants_str::ADMIN_RULE_ID_PLACEHOLDER,
            ),
            (
                crate::admin_data_table::AdminDataTable::Users,
                crate::admin_frontend_path::AdminFrontendPath::UsersRead,
                constants_str::ADMIN_USER_ID_PLACEHOLDER,
            ),
            (
                crate::admin_data_table::AdminDataTable::Roles,
                crate::admin_frontend_path::AdminFrontendPath::RolesRead,
                constants_str::ADMIN_ROLE_ID_PLACEHOLDER,
            ),
            (
                crate::admin_data_table::AdminDataTable::UserRoles,
                crate::admin_frontend_path::AdminFrontendPath::UserRolesRead,
                constants_str::ADMIN_USER_ROLE_ID_PLACEHOLDER,
            ),
        ]
        .into_iter()
        .for_each(|(admin_data_table, admin_frontend_path, placeholder)| {
            let section = admin_data_table.frontend_path();
            let detail = admin_frontend_path
                .get()
                .replace(placeholder, &constants_i64::ONE.to_string());
            assert!(bool::from(
                super::AdminPagePathRef::from(section.as_ref()).is_navigation_section(&section)
            ));
            assert!(bool::from(
                super::AdminPagePathRef::from(detail.as_str()).is_navigation_section(&section)
            ));
            let adjacent = format!("{}{}", section, constants_str::SQL_NAMES_ID);
            assert!(!bool::from(
                super::AdminPagePathRef::from(adjacent.as_str()).is_navigation_section(&section)
            ));
            crate::admin_data_table::AdminDataTable::PG_ORDER
                .into_iter()
                .filter(|other| *other != admin_data_table)
                .for_each(|other| {
                    assert!(!bool::from(
                        super::AdminPagePathRef::from(detail.as_str())
                            .is_navigation_section(&other.frontend_path())
                    ));
                });
        });
    }
}
