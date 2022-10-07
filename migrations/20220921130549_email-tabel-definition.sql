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
-- Name: emails; Type: TABLE; Schema: public; Owner: opeolluwa
--

CREATE TABLE public.emails (
    id uuid NOT NULL,
    sender_name character varying,
    sender_email character varying,
    email_subject character varying,
    email_body character varying,
    reply json[],
    star boolean DEFAULT false,
    date_sent date,
    fk_user_id uuid NOT NULL
);


ALTER TABLE public.emails OWNER TO opeolluwa;

--
-- Data for Name: emails; Type: TABLE DATA; Schema: public; Owner: opeolluwa


--
-- Name: emails emails_pkey; Type: CONSTRAINT; Schema: public; Owner: opeolluwa
--

ALTER TABLE ONLY public.emails
    ADD CONSTRAINT emails_pkey PRIMARY KEY (id);


--
-- Name: emails fk_user_emails; Type: FK CONSTRAINT; Schema: public; Owner: opeolluwa
--

ALTER TABLE ONLY public.emails
    ADD CONSTRAINT fk_user_emails FOREIGN KEY (fk_user_id) REFERENCES public.user_information(id);


--
-- PostgreSQL database dump complete
--

