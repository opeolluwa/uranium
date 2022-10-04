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
-- Name: notes; Type: TABLE; Schema: public; Owner: opeolluwa
--

CREATE TABLE public.notes (
    id uuid NOT NULL,
    title character varying(300) NOT NULL,
    description text,
    date_added date DEFAULT now(),
    last_update date DEFAULT now()
);


ALTER TABLE public.notes OWNER TO opeolluwa;

--
-- Data for Name: notes; Type: TABLE DATA; Schema: public; Owner: opeolluwa
--

COPY public.notes (id, title, description, date_added, last_update) FROM stdin;
\.


--
-- Name: notes notes_pkey; Type: CONSTRAINT; Schema: public; Owner: opeolluwa
--

ALTER TABLE ONLY public.notes
    ADD CONSTRAINT notes_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

