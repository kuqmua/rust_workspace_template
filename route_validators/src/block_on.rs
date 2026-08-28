use super::{TestPollCount, increment_block_on_poll_count, is_block_on_poll_limit_reached};

pub(crate) fn block_on<T>(input_future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(input_future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    let mut poll_count = TestPollCount::from(constants_usize::ZERO);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => {
                return output;
            }
            std::task::Poll::Pending => {
                assert!(
                    !is_block_on_poll_limit_reached(poll_count),
                    "{} super::block_on exceeded poll limit",
                    constants_str::ROUTE_VALIDATORS_BLOCK_ON_POLL_LIMIT_ER_ID
                );
                increment_block_on_poll_count(&mut poll_count);
                std::thread::yield_now();
            }
        }
    }
}
