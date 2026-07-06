use std::process::abort;
pub(crate) fn panic_if_err<T, E>(res: Result<T, E>, mk_panic_msg: impl FnOnce(E) -> String) -> T {
    match res {
        Ok(ok_v) => ok_v,
        Err(er) => {
            eprintln!("{}", mk_panic_msg(er));
            abort();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::panic_if_err;
    #[test]
    fn panic_if_err_returns_ok_value() {
        let value = panic_if_err::<u8, u16>(Ok(7), |_| String::from("unused"));
        assert_eq!(value, 7);
    }
}
