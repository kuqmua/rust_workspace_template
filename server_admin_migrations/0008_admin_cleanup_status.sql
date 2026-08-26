CREATE TABLE IF NOT EXISTS admin_cleanup_status (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    last_success_at TIMESTAMPTZ NOT NULL,
    last_deleted_rows BIGINT NOT NULL CHECK (last_deleted_rows >= 0)
);
