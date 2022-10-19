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
-- Name: todo_list; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.todo_list (
    id uuid UNIQUE NOT NULL,
    title character varying(300) NOT NULL,
    description text,
    last_update date DEFAULT now(),
    fk_user_id uuid NOT NULL,
    priority character varying(20),
    date_added timestamp without time zone DEFAULT now()
);


ALTER TABLE public.todo_list OWNER TO postgres;

--
-- Data for Name: todo_list; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.todo_list (id, title, description, last_update, fk_user_id, priority, date_added) FROM stdin;
d01201f3-d57b-4f0a-9ced-bc452d29e407	not urgent	not urgent task	2022-10-17	6bc97264-b297-44c9-a200-899847f0e442	not-urgent	2022-10-17 04:22:29.060234
b8d74db6-1b3d-4b40-b2d9-493ff8587636	Urgent	urgent task	2022-10-17	6bc97264-b297-44c9-a200-899847f0e442	urgent	2022-10-17 04:22:52.17802
a80f2266-c8fb-46b7-8b13-424d4c28cae6	Basic task	basic task, no priorities	2022-10-17	6bc97264-b297-44c9-a200-899847f0e442	normal	2022-10-17 04:23:29.689742
243a3ab8-7ba2-45da-90d4-2b51db456fb0	important	important	2022-10-17	6bc97264-b297-44c9-a200-899847f0e442	delete	2022-10-17 04:23:50.218194
b1836b58-a1dd-4590-a899-35a64311617c	delicate cask	very very delicate	2022-10-17	6bc97264-b297-44c9-a200-899847f0e442	delicate	2022-10-17 04:24:18.508175
6c7b80ef-49a5-4ede-a1aa-8c6bd94dab63	urgent 	urgent	2022-10-17	6bc97264-b297-44c9-a200-899847f0e442	urgent	2022-10-17 09:04:35.902842
\.


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

