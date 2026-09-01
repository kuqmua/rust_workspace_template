#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct AdminPageRange {
    end: server_admin_contract::admin_page_total::AdminPageTotal,
    start: server_admin_contract::admin_page_total::AdminPageTotal,
    next_offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    previous_offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    next_disabled: super::admin_page_nav_disabled::AdminPageNavDisabled,
    previous_disabled: super::admin_page_nav_disabled::AdminPageNavDisabled,
}

impl AdminPageRange {
    #[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
    pub(crate) fn new(
        offset: server_admin_contract::admin_page_offset::AdminPageOffset,
        limit: server_admin_contract::admin_page_limit::AdminPageLimit,
        total: server_admin_contract::admin_page_total::AdminPageTotal,
    ) -> Self {
        let offset_value = u32::from(offset);
        let limit_value = u16::from(limit);
        let total_value = u64::from(total);
        let previous_offset = offset_value.saturating_sub(u32::from(limit_value));
        let next_offset = offset_value.saturating_add(u32::from(limit_value));
        Self {
            end: server_admin_contract::admin_page_total::AdminPageTotal::from(
                u64::from(offset_value)
                    .saturating_add(u64::from(limit_value))
                    .min(total_value),
            ),
            next_disabled: super::admin_page_nav_disabled::AdminPageNavDisabled::from(
                u64::from(next_offset) >= total_value,
            ),
            next_offset: server_admin_contract::admin_page_offset::AdminPageOffset::from(
                next_offset,
            ),
            previous_disabled: super::admin_page_nav_disabled::AdminPageNavDisabled::from(
                offset_value == constants_u32::ZERO,
            ),
            previous_offset: server_admin_contract::admin_page_offset::AdminPageOffset::from(
                previous_offset,
            ),
            start: server_admin_contract::admin_page_total::AdminPageTotal::from(
                u64::from(offset_value)
                    .saturating_add(1u64)
                    .min(total_value),
            ),
        }
    }

    pub(crate) const fn end(self) -> server_admin_contract::admin_page_total::AdminPageTotal {
        self.end
    }

    pub(crate) const fn next_disabled(
        self,
    ) -> super::admin_page_nav_disabled::AdminPageNavDisabled {
        self.next_disabled
    }

    pub(crate) const fn next_offset(
        self,
    ) -> server_admin_contract::admin_page_offset::AdminPageOffset {
        self.next_offset
    }

    pub(crate) const fn previous_disabled(
        self,
    ) -> super::admin_page_nav_disabled::AdminPageNavDisabled {
        self.previous_disabled
    }

    pub(crate) const fn previous_offset(
        self,
    ) -> server_admin_contract::admin_page_offset::AdminPageOffset {
        self.previous_offset
    }

    pub(crate) const fn start(self) -> server_admin_contract::admin_page_total::AdminPageTotal {
        self.start
    }
}

#[cfg(test)]
mod tests {
    fn page_range(offset: u32, limit: u16, total: u64) -> super::AdminPageRange {
        let Ok(limit) = server_admin_contract::admin_page_limit::AdminPageLimit::try_from(limit)
        else {
            std::panic::panic_any(constants_str::PANIC_1543EFB0);
        };
        super::AdminPageRange::new(
            server_admin_contract::admin_page_offset::AdminPageOffset::from(offset),
            limit,
            server_admin_contract::admin_page_total::AdminPageTotal::from(total),
        )
    }

    #[test]
    fn test_page_range_supports_empty_and_first_pages() {
        let empty = page_range(constants_u32::ZERO, 20u16, constants_u64::ZERO);
        assert_eq!(u64::from(empty.start()), constants_u64::ZERO);
        assert_eq!(u64::from(empty.end()), constants_u64::ZERO);
        assert!(bool::from(empty.previous_disabled()));
        assert!(bool::from(empty.next_disabled()));

        let first = page_range(constants_u32::ZERO, 20u16, 41u64);
        assert_eq!(u64::from(first.start()), 1u64);
        assert_eq!(u64::from(first.end()), 20u64);
        assert_eq!(u32::from(first.next_offset()), 20u32);
        assert!(!bool::from(first.next_disabled()));
    }

    #[test]
    fn test_page_range_supports_partial_out_of_range_and_overflow_pages() {
        let partial = page_range(40u32, 20u16, 41u64);
        assert_eq!(u64::from(partial.start()), 41u64);
        assert_eq!(u64::from(partial.end()), 41u64);
        assert_eq!(u32::from(partial.previous_offset()), 20u32);
        assert!(bool::from(partial.next_disabled()));

        let out_of_range = page_range(80u32, 20u16, 41u64);
        assert_eq!(u64::from(out_of_range.start()), 41u64);
        assert_eq!(u64::from(out_of_range.end()), 41u64);

        let overflow = page_range(u32::MAX, 100u16, u64::MAX);
        assert_eq!(u32::from(overflow.next_offset()), u32::MAX);
        assert_eq!(u64::from(overflow.start()), u64::from(u32::MAX) + 1u64);
        assert_eq!(u64::from(overflow.end()), u64::from(u32::MAX) + 100u64);
    }
}
