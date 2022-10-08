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

SET default_table_access_method = heap;

--
-- Name: todo_list; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.todo_list (
    id uuid NOT NULL,
    title character varying(300) NOT NULL,
    description text,
    date_added date DEFAULT now(),
    last_update date DEFAULT now(),
    fk_user_id uuid NOT NULL
);


ALTER TABLE public.todo_list OWNER TO postgres;

--
-- Name: todo_list todo_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.todo_list
    ADD CONSTRAINT todo_id_key UNIQUE (id);


--
-- Name: TABLE todo_list; Type: ACL; Schema: public; Owner: postgres
--

GRANT ALL ON TABLE public.todo_list TO opeolluwa;


--
-- PostgreSQL database dump complete
--

