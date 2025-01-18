CREATE TABLE tweets (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    message TEXT NOT NULL
);
