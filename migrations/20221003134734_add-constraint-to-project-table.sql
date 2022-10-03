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
-- Name: project_information; Type: TABLE; Schema: public; Owner: opeolluwa
--

CREATE TABLE public.project_information (
    id uuid NOT NULL,
    name character varying NOT NULL,
    description character varying NOT NULL,
    date_added date DEFAULT now() NOT NULL,
    technologies_used character varying[] NOT NULL,
    repo_url character varying,
    live_url character varying
);


ALTER TABLE public.project_information OWNER TO opeolluwa;

--
-- Data for Name: project_information; Type: TABLE DATA; Schema: public; Owner: opeolluwa
--

COPY public.project_information (id, name, description, date_added, technologies_used, repo_url, live_url) FROM stdin;
\.


--
-- Name: project_information unique_live_url; Type: CONSTRAINT; Schema: public; Owner: opeolluwa
--

ALTER TABLE ONLY public.project_information
    ADD CONSTRAINT unique_live_url UNIQUE (live_url);


--
-- Name: project_information unique_project_name; Type: CONSTRAINT; Schema: public; Owner: opeolluwa
--

ALTER TABLE ONLY public.project_information
    ADD CONSTRAINT unique_project_name UNIQUE (name);


--
-- Name: project_information unique_repo_url; Type: CONSTRAINT; Schema: public; Owner: opeolluwa
--

ALTER TABLE ONLY public.project_information
    ADD CONSTRAINT unique_repo_url UNIQUE (repo_url);


--
-- PostgreSQL database dump complete
--

