use super::{MAX_BLOCK_ON_POLLS, TestPollCount, TestPollLimitReached};

pub(super) fn is_block_on_poll_limit_reached(poll_count: TestPollCount) -> TestPollLimitReached {
    TestPollLimitReached::from(poll_count.0 >= MAX_BLOCK_ON_POLLS)
}
