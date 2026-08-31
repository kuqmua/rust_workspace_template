pub fn make_query_bind_error<Source>(
    source: Source,
) -> crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError
where
    Source: std::error::Error + Send + Sync + 'static,
{
    let boxed: sqlx::error::BoxDynError = Box::new(source);
    crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(boxed)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_query_bind_error_preserves_its_source() {
        let error = crate::make_query_bind_error::make_query_bind_error(std::io::Error::other(
            constants_str::ERROR,
        ));
        let source = std::error::Error::source(&error)
            .expect("c9d460e5 query_bind_error_preserves_its_source invariant must hold");

        assert_eq!(source.to_string(), constants_str::ERROR);
        assert_eq!(
            error.to_string(),
            "failed to bind PostgreSQL query parameter"
        );
        assert_eq!(
            source
                .source()
                .expect("4e5bcc6b query_bind_error_preserves_its_source invariant must hold")
                .to_string(),
            constants_str::ERROR
        );
    }
}
