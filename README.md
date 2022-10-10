# nitride
An admin dashboard built on  [Vue.js](https://vuejs.org/), [TypeScript](https://www.typescriptlang.org/), [Rust](https://www.rust-lang.org/), and [PostgreSQL](https://www.postgresql.org/).

See the User Interface source code [here](https://github.com/opeolluwa/nitride-ui).


## Requirement
- [Rust](https://www.rust-lang.org/tools/install) v1.63 or greater 
- [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14.5 or greater
- [sqlx](https://crates.io/crates/sqlx) for interacting with the database,
- [sqlx-cli](https://crates.io/crates/sqlx-cli), a command line tool for sqlx,
- [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change,

## Installation (development)

1. clone Repository
2. copy and populate .env.example to .env `cp .env.example .env`, paying attention to the `SECRET` and `DATABASE_URL` variables
3. Run `sqlx database create` to create the database from the specified `DATABASE_URL` Use `sqlx database drop` to revert the change
4. Run `sqlx migrate run` to run the migrations
5. use `sqlx migrate add <migration_name>` to add a new migration


##  Documentation 
-  [API Documentation](https://documenter.getpostman.com/view/22658417/2s83zgv5nW) 
- [Application (logic) Documentation](https://opeolluwa.github.io/nitride/) 


## Demo 
The following credentials are set up for the [demo application](https://nitride.onrender.com/)
- EMAIL:  `guest@mailer.com`
- PASSWORD:  `guest`
- FULLNAME:  `Guest Username`,
- USERNAME:  `guest`