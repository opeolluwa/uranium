-- Add migration script here
CREATE TABLE public.user_information (
    id uuid NOT NULL,
    email character varying NOT NULL,
    password character varying NOT NULL,
    username character varying,
    fullname character varying,
    last_login DATE DEFAULT NOW()
);