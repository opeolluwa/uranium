## What is Racoon?
Racoon is an experimental Identity and Access Management Service for Microservices.

 The project is research work that focuses on enterprise-scale identity management for Microservices inspired by Google Applications Authentication Strategies

The solution is implemented with [the Rust programming language](https://www.rust-lang.org/tools/install) a memory-safe systems programming language for building fast and secure applications. The application also integrates with the [PostgreSQL](https://www.postgresql.org/download/)
  database which is optimized for fast read and write database operations

  As of this stage of development, the project uses [sqlx]() to interact with databases.  However, the project hope to ship with [xorm](https://github.com/opeolluwa/xorm) which is an async-first ORM library for Rust which make using ORM very simplistic