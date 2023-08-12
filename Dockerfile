FROM rust:1.71-alpine3.17 as builder
WORKDIR /usr/src/app
COPY . .


RUN apk add protoc musl-dev

RUN cargo build --release

RUN rm ./target/release/deps
RUN rm ./target/release/build



FROM rust:1.71-alpine3.17 as runner

WORKDIR /usr/src/app

COPY --from=builder /app/target/release/uranium ./uranium


EXPOSE 8000

CMD ["./uranium"]