-- Add migration script here
ALTER TABLE  public.emails
RENAME folder TO email_folder;