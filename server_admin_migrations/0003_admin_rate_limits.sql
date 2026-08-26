CREATE TABLE admin_rate_limits (
    scope TEXT NOT NULL,
    subject TEXT NOT NULL,
    window_started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    request_count BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (scope, subject),
    CONSTRAINT admin_rate_limits_scope_not_empty CHECK (char_length(scope) > 0),
    CONSTRAINT admin_rate_limits_subject_not_empty CHECK (char_length(subject) > 0),
    CONSTRAINT admin_rate_limits_request_count_nonnegative CHECK (request_count >= 0)
);
CREATE INDEX admin_rate_limits_window_idx ON admin_rate_limits (window_started_at);
