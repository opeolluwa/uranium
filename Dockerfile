ARG RUST_VERSION=1.82.0
ARG APP_NAME=bookmark

FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apt-get update && apt-get install -y protobuf-compiler musl-dev
COPY . .


RUN cargo build --release

FROM ubuntu AS final

COPY --from=build /app/target/release/bookmark ./server

EXPOSE 50051

CMD ["./server"]
