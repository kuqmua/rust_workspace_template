#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct AdminPageRange {
    #[getters(copy)]
    end: server_admin_contract::admin_page_total::AdminPageTotal,
    #[getters(copy)]
    start: server_admin_contract::admin_page_total::AdminPageTotal,
    #[getters(copy)]
    next_offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    #[getters(copy)]
    previous_offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    #[getters(copy)]
    next_disabled: super::admin_page_nav_disabled::AdminPageNavDisabled,
    #[getters(copy)]
    previous_disabled: super::admin_page_nav_disabled::AdminPageNavDisabled,
}

impl AdminPageRange {
    #[allow(
        clippy::single_call_fn,
        reason = "admin page range remains a named owner because its boundary role is clearer and directly testable"
    )]
    pub(crate) fn new(
        admin_page_offset: server_admin_contract::admin_page_offset::AdminPageOffset,
        admin_page_limit: server_admin_contract::admin_page_limit::AdminPageLimit,
        admin_page_total: server_admin_contract::admin_page_total::AdminPageTotal,
    ) -> Self {
        let offset_value = u32::from(admin_page_offset);
        let limit_value = u16::from(admin_page_limit);
        let total_value = u64::from(admin_page_total);
        let previous_offset = offset_value.saturating_sub(u32::from(limit_value));
        let next_offset = offset_value.saturating_add(u32::from(limit_value));
        let has_items = u64::from(offset_value) < total_value;
        Self {
            end: server_admin_contract::admin_page_total::AdminPageTotal::from(if has_items {
                u64::from(offset_value)
                    .saturating_add(u64::from(limit_value))
                    .min(total_value)
            } else {
                u64::from(constants_u32::ZERO)
            }),
            next_disabled: super::admin_page_nav_disabled::AdminPageNavDisabled::from(
                next_offset <= offset_value || u64::from(next_offset) >= total_value,
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
            start: server_admin_contract::admin_page_total::AdminPageTotal::from(if has_items {
                u64::from(offset_value).saturating_add(1u64)
            } else {
                u64::from(constants_u32::ZERO)
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    fn page_range(u32: u32, u16: u16, u64: u64) -> super::AdminPageRange {
        let Ok(limit) = server_admin_contract::admin_page_limit::AdminPageLimit::try_from(u16)
        else {
            std::panic::panic_any(constants_str::PANIC_1543EFB0);
        };
        super::AdminPageRange::new(
            server_admin_contract::admin_page_offset::AdminPageOffset::from(u32),
            limit,
            server_admin_contract::admin_page_total::AdminPageTotal::from(u64),
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
        assert_eq!(u64::from(out_of_range.start()), constants_u64::ZERO);
        assert_eq!(u64::from(out_of_range.end()), constants_u64::ZERO);

        let at_end = page_range(40u32, 20u16, 40u64);
        assert_eq!(u64::from(at_end.start()), constants_u64::ZERO);
        assert_eq!(u64::from(at_end.end()), constants_u64::ZERO);

        let overflow = page_range(u32::MAX, 100u16, u64::MAX);
        assert_eq!(u32::from(overflow.next_offset()), u32::MAX);
        assert!(bool::from(overflow.next_disabled()));
        assert_eq!(u64::from(overflow.start()), u64::from(u32::MAX) + 1u64);
        assert_eq!(u64::from(overflow.end()), u64::from(u32::MAX) + 100u64);
    }
    #[test]
    fn test_page_range_clamps_previous_offsets_and_preserves_near_maximum_progress() {
        assert!(
            [
                (1u32, 20u16, 41u64, 2u64, 21u64, 0u32, 21u32, false),
                (19u32, 20u16, 41u64, 20u64, 39u64, 0u32, 39u32, false),
                (20u32, 20u16, 40u64, 21u64, 40u64, 0u32, 40u32, true),
                (20u32, 20u16, 41u64, 21u64, 40u64, 0u32, 40u32, false),
                (
                    u32::MAX - 1u32,
                    1u16,
                    u64::MAX,
                    u64::from(u32::MAX),
                    u64::from(u32::MAX),
                    u32::MAX - 2u32,
                    u32::MAX,
                    false
                ),
                (
                    u32::MAX - 1u32,
                    2u16,
                    u64::MAX,
                    u64::from(u32::MAX),
                    u64::from(u32::MAX) + 1u64,
                    u32::MAX - 3u32,
                    u32::MAX,
                    false
                ),
            ]
            .into_iter()
            .all(
                |(offset, limit, total, start, end, previous, next, next_disabled)| {
                    let value = page_range(offset, limit, total);
                    u64::from(value.start()) == start
                        && u64::from(value.end()) == end
                        && u32::from(value.previous_offset()) == previous
                        && u32::from(value.next_offset()) == next
                        && !bool::from(value.previous_disabled())
                        && bool::from(value.next_disabled()) == next_disabled
                }
            )
        );
    }
}
