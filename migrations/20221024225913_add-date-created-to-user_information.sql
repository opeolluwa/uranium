-- Add migration script here
ALTER TABLE
    public.user_information
ADD
    COLUMN date_added DATE DEFAULT NOW();