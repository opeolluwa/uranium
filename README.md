# Raccoon

Open source Identity and access management service

## Table of Contents

- [Description](#description)
- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Support](#support)
- [License](#license)

## Description

Provide a more detailed explanation of what your project does, its purpose, and any key features or benefits it offers.

## Getting Started

To get started, clone the project and install dependencies

```sh
git clone https://github.com/opeolluwa/raccoon.git
cd raccoon
cargo build
```

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
mkdir bin
cargo build --release
cp target/release/raccoon ./raccoon/bin
```

## Support

Information on how to get help and support for the project, including contact details for your company's support team.

## License

This project is proprietary software owned by [Adeoye Adefemi](https://github.com/opeolluwa) and distribution under [MIT]() license. For inquiries, please contact
[Adeoye Adefemi](https://github.com/opeolluwa)
