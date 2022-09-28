--
-- PostgreSQL database dump
--

-- Dumped from database version 14.5 (Ubuntu 14.5-1.pgdg22.04+1)
-- Dumped by pg_dump version 14.5 (Ubuntu 14.5-1.pgdg22.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: authorization_tokens; Type: TABLE; Schema: public; Owner: opeolluwa
--

CREATE TABLE public.authorization_tokens (
    id uuid NOT NULL,
    token text NOT NULL,
    date_invalidated date DEFAULT now() NOT NULL
);


ALTER TABLE public.authorization_tokens OWNER TO opeolluwa;

--
-- Name: TABLE authorization_tokens; Type: COMMENT; Schema: public; Owner: opeolluwa
--

COMMENT ON TABLE public.authorization_tokens IS 'to implement logout ';


--
-- Data for Name: authorization_tokens; Type: TABLE DATA; Schema: public; Owner: opeolluwa
--

COPY public.authorization_tokens (id, token, date_invalidated) FROM stdin;
\.


--
-- Name: authorization_tokens authorization_tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: opeolluwa
--

ALTER TABLE ONLY public.authorization_tokens
    ADD CONSTRAINT authorization_tokens_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

