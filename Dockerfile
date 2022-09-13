FROM rust:1.62.0 as builder
WORKDIR /app
COPY . .


RUN cargo build --release
RUN rm ./target/release/deps/nitrogen/*


FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /app/target/release/nitrogen /app/nitrogen
COPY --from=builder /app/*.toml /app/

EXPOSE 8080
CMD ["/app/nitrogen"]