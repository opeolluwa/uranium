FROM rust:1.62.0 as builder
WORKDIR /app
COPY . .


RUN cargo build --release
RUN rm ./target/release/deps/nitride/*


FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /app/target/release/nitride /app/nitride
COPY --from=builder /app/*.toml /app/

EXPOSE 8080
CMD ["/app/nitride"]