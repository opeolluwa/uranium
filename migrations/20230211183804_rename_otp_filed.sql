-- Add migration script here
ALTER TABLE
    one_time_passwords RENAME COLUMN otp TO token;