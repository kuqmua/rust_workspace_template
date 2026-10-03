pub fn build_admin_cookie(
    admin_cookie_kind: crate::admin_cookie_kind::AdminCookieKind,
    std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    admin_cookie_max_age_seconds: crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds,
    runtime_admin_cookie_secure: crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure,
) -> Result<
    crate::std_admin_cookie::StdAdminCookie,
    crate::admin_secret_text_error::AdminSecretTextError,
> {
    let http_only = if matches!(
        admin_cookie_kind,
        crate::admin_cookie_kind::AdminCookieKind::Csrf
    ) {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    } else {
        constants_str::HTTPONLY
    };
    let secure_attr = if *runtime_admin_cookie_secure.get_inner() {
        constants_str::SECURE
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    crate::std_admin_cookie::StdAdminCookie::try_from(format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attr}",
        admin_cookie_kind.name().as_ref(),
        std_admin_str_ref.as_ref(),
        admin_cookie_max_age_seconds.get_inner()
    ))
    .map_err(crate::admin_secret_text_error::AdminSecretTextError::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cookie_text_bounds_include_all_attributes_for_each_cookie_kind() {
        assert!(
            [
                crate::admin_cookie_kind::AdminCookieKind::Access,
                crate::admin_cookie_kind::AdminCookieKind::Csrf,
                crate::admin_cookie_kind::AdminCookieKind::Refresh,
            ]
            .into_iter()
            .all(|kind| {
                [false, true].into_iter().all(|secure| {
                    [0u64, 60u64, u64::MAX].into_iter().all(|seconds| {
                        let build = |std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>| {
                            crate::build_admin_cookie::build_admin_cookie(
                                kind,
                                std_admin_str_ref,
                                crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(seconds),
                                crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(secure),
                            )
                        };
                        build(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX)).is_ok_and(|empty| {
                            [8191usize, 8192usize, 8193usize].into_iter().all(|total_length| {
                                let value = constants_str::X.repeat(total_length.saturating_sub(empty.as_ref().len()));
                                let result = build(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(value.as_str()));
                                if total_length > 8192usize {
                                    return result == Err(crate::admin_secret_text_error::AdminSecretTextError::TooLong);
                                }
                                result.is_ok_and(|cookie| {
                                    cookie.as_ref().len() == total_length
                                        && empty.as_ref().split_once('=').is_some_and(|(name, suffix)| {
                                            let mut expected = name.to_owned();
                                            expected.push('=');
                                            expected.push_str(&value);
                                            expected.push_str(suffix);
                                            cookie.as_ref().as_str() == expected
                                        })
                                })
                            })
                        })
                    })
                })
            })
        );
    }
}
