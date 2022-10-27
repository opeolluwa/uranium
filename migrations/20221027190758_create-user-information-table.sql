-- Add migration script here
CREATE TYPE account_status AS ENUM('active', 'inactive', 'deactivated');
CREATE TABLE public.user_information (
    id uuid NOT NULL,
    email character varying NOT NULL UNIQUE,
    password character varying NOT NULL,
    username character varying,
    fullname character varying,
    last_login DATE DEFAULT NOW(),
    date_added DATE DEFAULT NOW(),
    account_status account_status DEFAULT 'inactive',
    avatar TEXT 
);