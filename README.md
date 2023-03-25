# Raccoon 

_Authentication and Authorization Server for Microservices Architecture_

![racoon](./raccoon.png)


## Overview 
 This project is research work that focuses on enterprise-scale identity management for Microservices inspired by Google Applications Authentication Strategies

The solution is implemented with the [Rust programming language](https://www.rust-lang.org/tools/install) a memory-safe systems programming language for building fast and secure applications. The application also integrates with the [PostgreSQL](https://www.postgresql.org/download/)
  database which is optimized for fast read and write database operations. 


## Requirement
The following are required to run the application in development.
- [Rust](https://www.rust-lang.org/tools/install) v1.63 or greater
- [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14.5 or greater
- [sqlx](https://crates.io/crates/sqlx) for interacting with the database,
- [sqlx-cli](https://crates.io/crates/sqlx-cli), a command line tool for sqlx,
- [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change,

## Installation (development)
To run the application in development mode, follow the steps below
1. clone Repository
2. copy and populate .env.example to .env `cp .env.example .env`, paying attention to the `SECRET` and `DATABASE_URL` variables
3. Run `sqlx database create` to create the database from the specified `DATABASE_URL` Use `SQL database drop` to revert the change
4. Run `SQL migrate run` to run the migrations
5. use `SQL migrate add <migration_name>` to add a new migration


##  Documentation 
-  [API Documentation](https://documenter.getpostman.com/view/22658417/2s83zgv5nW) 
<!-- - [Application (logic) Documentation](https://opeolluwa.github.io/nitride/)  -->

## License 
The application is subjected to [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)
