CREATE TABLE notifications (
    id UUID PRIMARY KEY,
    message TEXT NOT NULL CHECK (char_length(message) BETWEEN 1 AND 4096),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
