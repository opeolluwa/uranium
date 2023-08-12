# Uranium

Central authentication/authorization for microservices using gRPC transport layer

- [Description](#description)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [Help](#help)
- [Authors](#authors)
- [Version History](#version-history)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Description

Uranium is an open-source central authentication &amp; authorization service built on the [Rust programming language](https://rust-lang.org) and SQL databases.

## Prerequisites

To run the application with minimal setup, [Docker](#) and [Docker compose](#) is required.
Otherwise, the run the application without docker, see the [Dependencies](#dependencies) section

## Getting Started

To get started as soon as possible

- clone the project `git clone https://github.com/opeolluwa/uranium `
- run `docker-compose up` to start the application

### Dependencies

- Rust
- ProgreSQL
- watchexec or cargo watch

### Installing

- How/where to download your program
- Any modifications needed to be made to files/folders

### Executing program

- How to run the program
- Step-by-step bullets

```
code blocks for commands
```

## Documentation

Describe any special instructions that are necessary to install a software package on your computer (if applicable).

### Executing the program


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

## Help

Any advise for common problems or issues.

```
command to run if program contains helper info
```

## Authors

Contributors names and contact info

ex. Dominique Pizzie  
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

- 0.2
  - Various bug fixes and optimizations
  - See [commit change]() or See [release history]()
- 0.1
  - Initial Release

## License

This project is licensed under the Apache License 2.0 License - see the [LICENSE](./LICENSE) file for details

## Acknowledgments

