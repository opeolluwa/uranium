-- Add migration script here
CREATE TABLE one_time_passwords(
    id UUID UNIQUE NOT NULL,
    otp VARCHAR,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT NOW(),
    expired_at TIMESTAMP WITHOUT TIME ZONE,
    is_expired BOOLEAN DEFAULT FALSE
)