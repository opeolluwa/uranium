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
install-dependencies: 
    @echo "Installing backend dependencies"
    cd backend && cargo build 

fmt:
    cargo fmt && cargo fix 

[group('watch')]
watch: 
    docker compose up -d
    docker logs -f app

[group('build')]
build:
    cargo run build --release 
