CREATE TABLE IF NOT EXISTS users (
    _id                 INTEGER         PRIMARY KEY AUTOINCREMENT,
    _created_at         TIMESTAMP       DEFAULT CURRENT_TIMESTAMP,
    _updated_at         TIMESTAMP       DEFAULT CURRENT_TIMESTAMP,
    is_active           BOOLEAN         DEFAULT 1,
    name                TEXT            NOT NULL,
    lastname            TEXT,
    gender              TEXT,
    date_of_birth       DATE,
    email               TEXT,
    phone_number        TEXT,
    address             TEXT,
    height              REAL            CHECK (height >= 0),
    weight              REAL            CHECK (weight >= 0),
    membership_type     TEXT,
    last_payment_date   DATE            DEFAULT CURRENT_TIMESTAMP,
    notes               TEXT
);

CREATE INDEX idx_users_active ON users(is_active);
CREATE INDEX idx_users_name ON users(name);