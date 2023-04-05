-- create access tokens table
CREATE TABLE public.access_tokens (
    id UUID NOT NULL,
    token CHARACTER VARYING NOT NULL,
    last_valid_at TIMESTAMP NOT NULL
);

-- create an index for the token
CREATE INDEX idx_access_token_token
ON public.access_tokens(token);
