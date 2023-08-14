FROM rust:1.71-alpine3.17 as builder
WORKDIR /usr/src/app
COPY . .


RUN apk add protoc musl-dev

RUN cargo build --release

#remove the source 
RUN rm -rf app
RUN rm -rf entity
RUN rm -rf migration
RUN rm -rf mailer
RUN rm -rf src



RUN rm -rf ./target/release/deps
RUN rm -rf ./target/release/build
RUN rm -rf ./target/release/examples
RUN rm -rf ./target/release/incremental
RUN rm -rf ./target/release/.fingerprint




FROM rust:1.71-alpine3.17 as runner

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/uranium .


EXPOSE 8000

CMD ["./uranium"]