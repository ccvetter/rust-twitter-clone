CREATE TABLE likes (
    id UUID PRIMARY KEY,
    tweet_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
