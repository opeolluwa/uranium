-- Add migration script here
ALTER TABLE
    public.emails
ALTER COLUMN
    date_sent TYPE timestamp without time zone;
ALTER TABLE
    public.emails
ALTER COLUMN
    date_sent SET DEFAULT NOW();