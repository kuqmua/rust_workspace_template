#![allow(clippy::shadow_reuse)]
const MAX_BLOCK_ON_POLLS: usize = 4096;
const BLOCK_ON_POLL_LIMIT_ER_ID: &str = "cf6e91ab";
const EXPECT_OK_ER_ID: &str = "db9d2f63";
const EXPECT_ER_ER_ID: &str = "2f755472";
const REPLACE_HEADER_MISSING_SRC_ER_ID: &str = "c3a0f7be";
pub(crate) struct TestExpId(pub &'static str);
impl From<&'static str> for TestExpId {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for TestExpId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
struct TestPanicText(pub &'static str);
impl From<&'static str> for TestPanicText {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for TestPanicText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
pub(crate) struct AxumTestHeaders(pub axum::http::HeaderMap);
impl std::ops::Deref for AxumTestHeaders {
    type Target = axum::http::HeaderMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<axum::http::HeaderMap> for AxumTestHeaders {
    fn as_ref(&self) -> &axum::http::HeaderMap {
        &self.0
    }
}
impl std::ops::DerefMut for AxumTestHeaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub(crate) struct AxumTestHeadersMutRef<'headers_lt>(pub &'headers_lt mut axum::http::HeaderMap);
impl<'headers_lt> From<&'headers_lt mut AxumTestHeaders> for AxumTestHeadersMutRef<'headers_lt> {
    fn from(value: &'headers_lt mut AxumTestHeaders) -> Self {
        Self(&mut value.0)
    }
}
impl<'headers_lt> From<&'headers_lt mut axum::http::HeaderMap>
    for AxumTestHeadersMutRef<'headers_lt>
{
    fn from(value: &'headers_lt mut axum::http::HeaderMap) -> Self {
        Self(value)
    }
}
pub(crate) struct AxumTestHeaderValue(pub axum::http::HeaderValue);
impl From<axum::http::HeaderValue> for AxumTestHeaderValue {
    fn from(value: axum::http::HeaderValue) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for AxumTestHeaderValue {
    type Target = axum::http::HeaderValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Copy)]
struct TestPollCount(pub usize);
struct TestPollLimitReached(pub bool);
impl std::ops::Not for TestPollLimitReached {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
#[allow(clippy::single_call_fn)] // shared insertion guard keeps header setup helpers consistent
fn insert_header_no_prev<'headers_lt, ValueTy>(
    headers: impl Into<AxumTestHeadersMutRef<'headers_lt>>,
    name: impl axum::http::header::IntoHeaderName,
    value: ValueTy,
) where
    ValueTy: Into<AxumTestHeaderValue>,
{
    let headers = headers.into();
    let prev = headers.0.insert(name, value.into().0);
    assert!(prev.is_none());
}
#[allow(clippy::single_call_fn)] // extracted to keep block_on loop hot path simple and reusable
const fn is_block_on_poll_limit_reached(poll_count: TestPollCount) -> TestPollLimitReached {
    TestPollLimitReached(poll_count.0 >= MAX_BLOCK_ON_POLLS)
}
#[allow(clippy::single_call_fn)] // keeps poll-count mutation centralized so block_on loop stays focused on state transitions
fn incr_block_on_poll_count(poll_count: &mut TestPollCount) {
    poll_count.0 = poll_count.0.saturating_add(1);
}
pub(crate) fn block_on<T>(input_future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(input_future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    let mut poll_count = TestPollCount(0usize);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => {
                return output;
            }
            std::task::Poll::Pending => {
                assert!(
                    !is_block_on_poll_limit_reached(poll_count),
                    "{BLOCK_ON_POLL_LIMIT_ER_ID} super::block_on exceeded poll limit"
                );
                incr_block_on_poll_count(&mut poll_count);
                std::thread::yield_now();
            }
        }
    }
}
#[track_caller]
pub(crate) fn panic_unexpected_variant(exp_id: impl Into<TestExpId>) -> ! {
    let exp_id = exp_id.into();
    panic!("4fe6f2e6 id={exp_id}");
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper keeps variant-mapping panic behavior consistent for owned and borrowed paths
fn map_or_panic_unexpected_variant<R>(map_res: Option<R>, exp_id: impl Into<TestExpId>) -> R {
    map_res.unwrap_or_else(|| panic_unexpected_variant(exp_id))
}
#[track_caller]
pub(crate) fn expect_variant<T, R>(
    v: T,
    map: impl FnOnce(T) -> Option<R>,
    exp_id: impl Into<TestExpId>,
) -> R {
    map_or_panic_unexpected_variant(map(v), exp_id)
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper centralizes borrowed-variant extraction with consistent panic path across test assertions
pub(crate) fn expect_variant_ref<T, R>(
    v: &T,
    map: impl FnOnce(&T) -> Option<R>,
    exp_id: impl Into<TestExpId>,
) -> R {
    map_or_panic_unexpected_variant(map(v), exp_id)
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared panic formatting keeps expectation failures consistent across helpers
fn panic_unexpected_result(
    er_id: impl Into<TestPanicText>,
    fn_name: impl Into<TestPanicText>,
    expected: impl Into<TestPanicText>,
    exp_id: impl Into<TestExpId>,
) -> ! {
    let er_id = er_id.into();
    let fn_name = fn_name.into();
    let expected = expected.into();
    let exp_id = exp_id.into();
    panic!("{er_id} unexpected {expected} for {fn_name}, id={exp_id}");
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared panic builder keeps explicit UUID-tagged panic path reusable for header-replace preconditions
fn panic_replace_header_missing_src(exp_id: impl Into<TestExpId>) -> ! {
    let exp_id = exp_id.into();
    panic!("{REPLACE_HEADER_MISSING_SRC_ER_ID} missing source header while replacing, id={exp_id}");
}
#[track_caller]
pub(crate) fn expect_ok<T, E>(v: Result<T, E>, exp_id: impl Into<TestExpId>) -> T {
    v.unwrap_or_else(|_| panic_unexpected_result(EXPECT_OK_ER_ID, "expect_ok", "Err", exp_id))
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper keeps ok-result equality assertions concise and consistent across validator tests
pub(crate) fn assert_ok_eq<T, E>(v: Result<T, E>, exp_id: impl Into<TestExpId>, expected: &T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_eq!(&expect_ok(v, exp_id), expected);
}
#[track_caller]
pub(crate) fn expect_er<T, E>(v: Result<T, E>, exp_id: impl Into<TestExpId>) -> E {
    v.err()
        .unwrap_or_else(|| panic_unexpected_result(EXPECT_ER_ER_ID, "expect_er", "Ok", exp_id))
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper centralizes error extraction and post-check mapping so higher-level helpers avoid repeating expect_er plumbing
fn map_err<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    check: impl FnOnce(&E),
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    let exp_id = exp_id.into();
    let er = expect_er(v, exp_id.0);
    check(&er);
    map(er, exp_id.0)
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared mapper avoids repeating expect_er + variant extraction boilerplate in tests
pub(crate) fn expect_er_mapped<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    map_err(v, exp_id, |_| (), map)
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper composes result extraction with variant mapping for concise validator tests
pub(crate) fn expect_er_variant<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    map: impl FnOnce(E) -> Option<R>,
) -> R {
    expect_er_mapped(v, exp_id, |er, mapped_exp_id| {
        expect_variant(er, map, mapped_exp_id)
    })
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper supports variant extraction without moving the error value in tests
pub(crate) fn expect_er_variant_ref<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    map: impl FnOnce(&E) -> Option<R>,
) -> R {
    expect_er_mapped(v, exp_id, |er, mapped_exp_id| {
        expect_variant_ref(&er, map, mapped_exp_id)
    })
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper composes status-code assertion with custom mapping to reduce repetition in variant helpers
fn map_err_after_status_check<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
    map: impl FnOnce(E, &'static str) -> R,
) -> R
where
    E: crate::GetAxumHttpStatusCode,
{
    map_err(
        v,
        exp_id,
        |er| {
            assert_eq!(er.get_axum_http_status_code(), expected);
        },
        map,
    )
}
#[track_caller]
pub(crate) fn assert_err_status_code<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
) -> E
where
    E: crate::GetAxumHttpStatusCode,
{
    map_err_after_status_check(v, exp_id, expected, |er, _| er)
}
#[track_caller]
pub(crate) fn assert_err_status_code_only<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
) where
    E: crate::GetAxumHttpStatusCode,
{
    drop(assert_err_status_code(v, exp_id, expected));
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper composes status-code assertion with variant mapping to reduce repetitive test boilerplate
pub(crate) fn assert_err_status_code_variant<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
    map: impl FnOnce(E) -> Option<R>,
) -> R
where
    E: crate::GetAxumHttpStatusCode,
{
    map_err_after_status_check(v, exp_id, expected, |er, mapped_exp_id| {
        expect_variant(er, map, mapped_exp_id)
    })
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper supports status+variant assertions while borrowing the error for field reads
pub(crate) fn assert_err_status_code_variant_ref<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: crate::GetAxumHttpStatusCode,
{
    map_err_after_status_check(v, exp_id, expected, |er, mapped_exp_id| {
        expect_variant_ref(&er, map, mapped_exp_id)
    })
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper lets tests reuse err-variant extraction with optional status checks without duplicating branching
pub(crate) fn expect_err_variant_ref_with_status<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: Option<crate::AxumHttpStatusCode>,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: crate::GetAxumHttpStatusCode,
{
    let exp_id = exp_id.into();
    match expected {
        Some(status_code) => assert_err_status_code_variant_ref(v, exp_id.0, status_code, map),
        None => expect_er_variant_ref(v, exp_id.0, map),
    }
}
pub(crate) fn mk_headers_with_entry<ValueTy>(
    name: impl axum::http::header::IntoHeaderName,
    value: ValueTy,
) -> AxumTestHeaders
where
    ValueTy: Into<AxumTestHeaderValue>,
{
    let mut headers = axum::http::HeaderMap::new();
    insert_header_no_prev(&mut headers, name, value);
    AxumTestHeaders(headers)
}
#[track_caller]
pub(crate) fn replace_header_name<'headers_lt>(
    headers: impl Into<AxumTestHeadersMutRef<'headers_lt>>,
    from_name: impl axum::http::header::AsHeaderName,
    to_name: impl axum::http::header::IntoHeaderName,
    exp_id: impl Into<TestExpId>,
) {
    let headers = headers.into();
    let value = headers
        .0
        .remove(from_name)
        .unwrap_or_else(|| panic_replace_header_missing_src(exp_id));
    insert_header_no_prev(headers.0, to_name, value);
}
pub(crate) fn non_utf8_header_value() -> AxumTestHeaderValue {
    AxumTestHeaderValue(axum::http::HeaderValue::from_bytes(&[0x80]).expect("86eb20cf"))
}
#[track_caller]
pub(crate) fn assert_panics(
    action: impl FnOnce() + std::panic::UnwindSafe,
    exp_id: impl Into<TestExpId>,
) {
    let exp_id = exp_id.into();
    let panic_res = std::panic::catch_unwind(action);
    drop(panic_res.expect_err(exp_id.0));
}
#[cfg(test)]
mod tests {
    #[test]
    fn block_on_panics_for_never_ready_future() {
        super::assert_panics(
            || {
                let _ignored =
                    super::block_on(std::future::poll_fn(|_| std::task::Poll::<u8>::Pending));
            },
            "1fc8c9f0",
        );
    }
    #[test]
    fn poll_limit_helper_returns_false_below_limit_and_true_at_limit() {
        assert!(!super::is_block_on_poll_limit_reached(
            super::TestPollCount(0)
        ));
        assert!(
            super::is_block_on_poll_limit_reached(super::TestPollCount(super::MAX_BLOCK_ON_POLLS))
                .0
        );
    }
    #[test]
    fn poll_count_increment_helper_increments_once() {
        let mut poll_count = super::TestPollCount(0usize);
        super::incr_block_on_poll_count(&mut poll_count);
        assert_eq!(poll_count.0, 1usize);
    }
    #[test]
    fn expect_ok_returns_inner_value() {
        let v = super::expect_ok::<u8, u16>(Ok(7), "4f607799");
        assert_eq!(v, 7);
    }
    #[test]
    fn assert_ok_eq_checks_ok_result_value() {
        super::assert_ok_eq::<u8, u16>(Ok(7), "9665f80a", &7);
    }
    #[test]
    fn expect_er_returns_inner_error() {
        let v = super::expect_er::<u8, u16>(Err(9), "5cd39e4b");
        assert_eq!(v, 9);
    }
    #[test]
    fn expect_er_mapped_passes_error_and_exp_id_to_mapper() {
        let v = super::expect_er_mapped::<u8, u16, (u16, &'static str)>(
            Err(9),
            "8ce7a316",
            |er, exp_id| (er, exp_id),
        );
        assert_eq!(v, (9, "8ce7a316"));
    }
    #[test]
    fn panic_unexpected_variant_always_panics() {
        super::assert_panics(|| super::panic_unexpected_variant("f66647ab"), "b6dba95d");
    }
    #[test]
    fn expect_variant_returns_mapped_value_for_matching_variant() {
        let v = super::expect_variant(Some(7u8), |v| v, "0dfd9a91");
        assert_eq!(v, 7);
    }
    #[test]
    fn expect_variant_ref_returns_mapped_value_for_matching_variant() {
        let value = Some(7u8);
        let v = super::expect_variant_ref(&value, |v| *v, "a2fcbad4");
        assert_eq!(v, 7);
    }
    #[test]
    fn expect_variant_panics_for_unexpected_variant() {
        super::assert_panics(
            || {
                let _: u8 = super::expect_variant::<Option<u8>, u8>(None, |v| v, "dba097b9");
            },
            "a9651f69",
        );
    }
    #[test]
    fn expect_er_variant_maps_matching_error_variant() {
        #[derive(std::fmt::Debug)]
        enum TestEr {
            A(u8),
        }
        let v =
            super::expect_er_variant::<(), TestEr, u8>(
                Err(TestEr::A(3)),
                "9bf4ce17",
                |er| match er {
                    TestEr::A(v) => Some(v),
                },
            );
        assert_eq!(v, 3);
    }
    #[test]
    fn expect_er_variant_ref_maps_matching_error_variant_without_move() {
        #[derive(std::fmt::Debug)]
        enum TestEr {
            A(u8),
        }
        let v =
            super::expect_er_variant_ref::<(), TestEr, u8>(Err(TestEr::A(3)), "8dfc4389", |er| {
                match er {
                    TestEr::A(v) => Some(*v),
                }
            });
        assert_eq!(v, 3);
    }
    #[test]
    fn assert_err_status_code_variant_checks_status_and_extracts_variant() {
        #[derive(std::fmt::Debug)]
        enum TestEr {
            A,
        }
        impl crate::GetAxumHttpStatusCode for TestEr {
            const AXUM_HTTP_STATUS_CODE: crate::AxumHttpStatusCode =
                crate::AxumHttpStatusCode::BAD_REQUEST;
        }
        let _: () = super::assert_err_status_code_variant::<(), TestEr, ()>(
            Err(TestEr::A),
            "c1d74a8e",
            crate::AxumHttpStatusCode::BAD_REQUEST,
            |er| match er {
                TestEr::A => Some(()),
            },
        );
    }
    #[test]
    fn assert_err_status_code_variant_ref_checks_status_and_extracts_variant_without_move() {
        #[derive(std::fmt::Debug)]
        enum TestEr {
            A(u8),
        }
        impl crate::GetAxumHttpStatusCode for TestEr {
            const AXUM_HTTP_STATUS_CODE: crate::AxumHttpStatusCode =
                crate::AxumHttpStatusCode::BAD_REQUEST;
        }
        let v = super::assert_err_status_code_variant_ref::<(), TestEr, u8>(
            Err(TestEr::A(7)),
            "8afb4ffd",
            crate::AxumHttpStatusCode::BAD_REQUEST,
            |er| match er {
                TestEr::A(v) => Some(*v),
            },
        );
        assert_eq!(v, 7);
    }
    #[test]
    fn mk_headers_with_entry_inserts_value_for_case_insensitive_name() {
        let headers = super::mk_headers_with_entry(
            "Commit",
            axum::http::HeaderValue::from_static("deadbeef"),
        );
        let actual = headers.get("commit");
        assert_eq!(
            actual,
            Some(&axum::http::HeaderValue::from_static("deadbeef"))
        );
    }
    #[test]
    fn replace_header_name_moves_value_to_new_key() {
        let mut headers = super::mk_headers_with_entry(
            "x-commit",
            axum::http::HeaderValue::from_static("deadbeef"),
        );
        super::replace_header_name(
            &mut headers,
            "x-commit",
            axum::http::HeaderName::from_static("commit"),
            "348c0e57",
        );
        assert!(headers.get("x-commit").is_none());
        assert_eq!(
            headers.get("commit"),
            Some(&axum::http::HeaderValue::from_static("deadbeef"))
        );
    }
    #[test]
    fn non_utf8_header_value_creates_non_utf8_header() {
        assert_eq!(
            super::non_utf8_header_value().to_str().err().map(|_| true),
            Some(true)
        );
    }
    #[test]
    fn assert_err_status_code_returns_error_after_status_check() {
        #[derive(std::fmt::Debug)]
        struct TestErr;
        impl crate::GetAxumHttpStatusCode for TestErr {
            const AXUM_HTTP_STATUS_CODE: crate::AxumHttpStatusCode =
                crate::AxumHttpStatusCode::BAD_REQUEST;
        }
        let _err = super::assert_err_status_code::<(), TestErr>(
            Err(TestErr),
            "4a1791d2",
            crate::AxumHttpStatusCode::BAD_REQUEST,
        );
        super::assert_err_status_code_only::<(), TestErr>(
            Err(TestErr),
            "773c5af2",
            crate::AxumHttpStatusCode::BAD_REQUEST,
        );
    }
}
