pub(crate) fn panic_if_err<T, E>(
    res: Result<T, E>,
    mk_panic_message: impl FnOnce(E) -> String,
) -> T {
    match res {
        Ok(ok_v) => ok_v,
        Err(error) => {
            eprintln!("{}", mk_panic_message(error));
            std::process::abort();
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn panic_if_err_returns_ok_value() {
        let value =
            super::panic_if_err::<u8, u16>(Ok(7), |_| String::from(str_constants::text::UNUSED));
        assert_eq!(value, 7);
    }
}
