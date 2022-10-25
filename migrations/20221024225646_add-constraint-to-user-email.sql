-- Add migration script here
ALTER TABLE
    public.user_information
ADD
    CONSTRAINT user_information_email_key UNIQUE (email);