DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type WHERE typname = 'operation'
    ) THEN
        CREATE TYPE operation AS ENUM (
            'deposit',
            'withdrawal',
            'transfer'
        );
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    operation operation NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    from_id UUID,
    to_id UUID,

    CONSTRAINT fk_to_user
        FOREIGN KEY (from_id)
        REFERENCES accounts(id),
    
    CONSTRAINT fk_from_user
        FOREIGN KEY (to_id)
        REFERENCES accounts(id)
);


CREATE TABLE IF NOT EXISTS refresh_tokens (
    refresh_token_hash TEXT UNIQUE PRIMARY KEY,
    user_id UUID NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_refresh_tokens_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
