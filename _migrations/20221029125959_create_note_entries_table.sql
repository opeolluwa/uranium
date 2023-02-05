-- Add migration script here
CREATE TABLE note_entries(
    id UUID UNIQUE NOT NULL,
    title CHARACTER VARYING(300) NOT NULL,
    content TEXT,
    user_id UUID REFERENCES user_information (id),
    category CHARACTER VARYING(20),
    date_added TIMESTAMP WITHOUT TIME ZONE DEFAULT NOW(),
    last_updated TIMESTAMP WITHOUT TIME ZONE DEFAULT NOW()
)