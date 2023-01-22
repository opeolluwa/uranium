# RACOON
![racoon](./racoon.png)

Authentication and Authorization Server for Microservices Architecture

## Requirement
The following are required to run the application in development or in production environment
- [Rust](https://www.rust-lang.org/tools/install) v1.63 or greater 
- [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14.5 or greater
- [sqlx](https://crates.io/crates/sqlx) for interacting with the database,
- [sqlx-cli](https://crates.io/crates/sqlx-cli), a command line tool for sqlx,
- [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change,

## Installation (development)
To run the application in development mode, follow the steps below
1. clone Repository
2. copy and populate .env.example to .env `cp .env.example .env`, paying attention to the `SECRET` and `DATABASE_URL` variables
3. Run `sqlx database create` to create the database from the specified `DATABASE_URL` Use `sqlx database drop` to revert the change
4. Run `sqlx migrate run` to run the migrations
5. use `SQL migrate add <migration_name>` to add a new migration


##  Documentation 
-  [API Documentation](https://documenter.getpostman.com/view/22658417/2s83zgv5nW) 
- [Application (logic) Documentation](https://opeolluwa.github.io/nitride/) 
