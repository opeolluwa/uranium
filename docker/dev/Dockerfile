FROM rust:1.86.0-slim-bullseye

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

EXPOSE 50051

CMD ["cargo", "watch", "-qcx", "run "]
