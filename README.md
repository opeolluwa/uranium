# Uranium

A powerful tool built in Rust that empowers frontend developers to seamlessly build a functional full-stack application with a fast and secure backend. With Uranium, developers can simply grab the project SDK and effortlessly integrate it into their web, mobile, or desktop application's client-side code.

## Table of Contents

- [Description](#description)
- [Features](#features)
- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Svelte](https://svelte.dev/)
- [Support](#support)
- [License](#license)

## Overview

## Features

- **Fast and Secure Backend**: Uranium provides a high-performance and secure backend infrastructure, ensuring the reliability and efficiency of your application.
- **Simplified Integration**: With the project SDK, integration into your existing client-side code becomes a breeze, enabling you to focus on building your application's unique features.
- **Full-Stack Functionality**: Uranium empowers front-end developers to handle both the client-side and server-side aspects of their applications, enabling end-to-end control and flexibility.
- **Cross-Platform Compatibility**: Build applications for the web, mobile, or desktop platforms using Uranium, enabling your projects to reach a broader audience.

## Getting Started

Integration guides are planned for the following technologies: see the road map for progress

- [React](https://reactjs.org/)
- [React Native](https://reactnative.dev/)
- [Electron](https://www.electronjs.org/)
- [Flutter](https://flutter.dev/)
- [Vue](https://vuejs.org/)
- [Python](https://www.python.org/)
- [Rust](https://www.rust-lang.org/)
- [Deno](https://deno.land/)

## Prerequisites

The following are required to run the application in development.

- [Rust](https://www.rust-lang.org/tools/install) v1.63 or greater
- [PostgreSQL](https://www.postgresql.org/download/) PostgreSQL server v14.5 or greater
- [sqlx](https://crates.io/crates/sqlx) for interacting with the database,
- [sqlx-cli](https://crates.io/crates/sqlx-cli), a command line tool for sqlx,
- [cargo watch](https://crates.io/crates/cargo-watch), a tool for watching the project files and recompiling when they change

## Installation

Instructions on how to install and set up the project.

## Usage

Run the following command in the project root to execute the annotated command

- run the project

```sh
cargo run
```

- build the project and copy it to bin directory

```sh
mkdir bin # make a directory ot keep the binary
cargo build --release # build the application
cp target/release/uranium ./bin/app # copy the executable into  the created folder
./app # run the app on Linux based system or open app.exe on Windows Operating System
```

- run the application in development mode

```sh
cargo watch -x run
```

- run the project in release mode

```sh
cargo run --release
```

- run unit tests

```sh
cargo test
```

## Road map

- [ ] Build the core server
- [ ] Add support for custom databases such as `MySQL`, `SQLite` etc...
- [ ] Add SDK for `Python`
- [ ] Add support to `Node.js`
- [ ] Add SDK for `Rust`
- [ ] Add support for other languages such as `Go`, `Java` etc...
- [ ] Add support for other frameworks such as `React`, `Vue`, `Svelte` etc...
- [ ] Add support for other platforms such as `Flutter`, `Electron` etc...
- [ ] Integrate Admin dashboard

## Support

If you have any questions about this repository, please join the [discussion](https://github.com/opeolluwa/uranium/discussions)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
