-- Add migration script here
ALTER TABLE user_information RENAME COLUMN last_available_on TO last_available_at;