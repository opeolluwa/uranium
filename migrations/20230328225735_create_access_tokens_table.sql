-- create access tokens table
CREATE TABLE public.access_tokens (
    id UUID UNIQUE NOT NULL,
    token CHARACTER VARYING NOT NULL,
    is_blacklisted BOOLEAN NOT NULL
);
