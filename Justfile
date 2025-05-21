alias w:= watch
alias b:= build
alias install := install-dependencies

set dotenv-required
set dotenv-load := true
set dotenv-path := "./.env.local"
set export :=  true

default: 
    @just --list --list-heading $'Available commands\n'

[doc('Install the application dependencies')]
init platform: 
    @echo "preparing "
  


fmt:
    cargo fmt && cargo fix 

[group('watch')]
watch: 
    cargo watch -qcx run 

[group('build')]
build:
    cargo run build --release 
