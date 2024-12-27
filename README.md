# Uranium Core

Uranium is a free nd cistomizable central authorization system. It is made of
supporting code bases, also built in Rust.

_⚠️ Please note that the project is currently 90% done_

## Getting started

To run the appliction locally, ensure the following prerequsites are made
available

- [Rust](https://rust-lang.org)
- [just](https://just.systems)
- [Docker](https://docker.com)

### Local development

The quickest way to get started is to use the
[Uranum CLI](https://crates.io/uranium_cli),

_⚠️ The CLI is a work in progress_

```sh
ucli new <path>
```

Follow the prompt, this would clone the necessary repositories and create a
`.uranium` mich contains the Docker files and a bash script to run you
application from end to end.

### Related Repositories

- [Uranium gRPC Codegen](https://github.com/opeolluwa/uranium_grpc_codegen)
- [Uranium HTTP->GRPC proxy server](https://github.com/opeolluwa/uranium_proxy)
- [Uranium CLI](https://github.com/opeolluwa/uranium_cli)

## Help

For help and assitance, consider crating a discussion at
[https://github.com/opeolluwa/uranium/discussions](https://github.com/opeolluwa/uranium/discussions)
