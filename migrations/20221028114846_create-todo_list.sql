-- Add migration script here
CREATE TABLE public.todo_list (
   id UUID UNIQUE NOT NULL,
    title CHARACTER VARYING(300) NOT NULL,
    description TEXT,
    last_update DATE DEFAULT NOW(),
    user_id UUID REFERENCES user_information (id),
    priority CHARACTER VARYING(20),
    date_added timestamp without time zone DEFAULT NOW()
 
);