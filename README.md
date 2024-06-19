# Uranium

Free deployable central authorization system for small and enterprise scale application 

_⚠️ the application is a work in progress, please see the [roadmap](#roadmap) for the progress_
## Getting Started

To get started as soon as possible

- clone the project `git clone https://github.com/opeolluwa/uranium `
- run `docker-compose up` to start the application

### Dependencies

- Rust
- PostgreSQL
- Docker 
- Nodejs 

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

