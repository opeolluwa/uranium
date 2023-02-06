-- Add migration script here
-- Add migration script here
CREATE TYPE account_status AS ENUM('active', 'inactive', 'deactivated');
CREATE TABLE public.user_information (
    id UUID UNIQUE NOT NULL,
    email CHARACTER VARYING NOT NULL UNIQUE,
    password CHARACTER VARYING NOT NULL,
    username CHARACTER VARYING,
    fullname Character Varying,
    last_login DATE DEFAULT NOW(),
    date_added DATE DEFAULT NOW(),
    account_status account_status DEFAULT 'inactive',
    avatar TEXT 
);


