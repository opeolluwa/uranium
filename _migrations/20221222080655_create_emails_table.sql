-- Add migration script here
CREATE TYPE email_status AS ENUM('read', 'unread'); CREATE TYPE email_folder AS Enum('inbox', 'draft', 'important', 'trash', 'custom','sent'); CREATE TABLE public.emails (
    id UUID UNIQUE NOT NULL,
    sender_name Character Varying,
    sender_email CHARACTER VARYING NOT NULL,
    email_subject CHARACTER VARYING NOT NULL,
    email_body TEXT,
    date_sent DATE DEFAULT NOW(),
    is_starred BOOLEAN default false,
    folder email_folder DEFAULT 'inbox',
    is_archived BOOLEAN DEFAULT false,
    email_status email_status DEFAULT 'unread'
);