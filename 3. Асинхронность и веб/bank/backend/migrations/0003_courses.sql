CREATE TABLE IF NOT EXISTS courses (
    time_update_utc TIMESTAMPTZ NOT NULL PRIMARY KEY,
    base_code VARCHAR(3) NOT NULL,
    conversion_rates JSON NOT NULL
);
