-- create custome types
CREATE TYPE account_status AS ENUM('active', 'inactive', 'suspended', 'deactivated');CREATE TYPE gender AS ENUM('male', 'female', 'unspecified', 'others');--create the model
CREATE TABLE public.user_information (
    id UUID UNIQUE NOT NULL,
    firstname CHARACTER VARYING,
    lastname CHARACTER VARYING,
    middlename CHARACTER VARYING,
    username CHARACTER VARYING UNIQUE,
    email CHARACTER VARYING NOT NULL UNIQUE,
    password CHARACTER VARYING NOT NULL,
    account_status account_status DEFAULT 'inactive',
    date_of_birth DATE,
    gender gender,
    avatar CHARACTER VARYING,
    phone_number CHARACTER VARYING,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITHOUT TIME ZONE DEFAULT NOW(),
    last_available_on TIMESTAMP WITHOUT TIME ZONE DEFAULT NOW()
);