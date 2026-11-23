FROM rust:1.97

ENV CARGO_TARGET_DIR=/cargo-target 

RUN set -e && \
    apt-get update -q && \
    apt-get install -yq \
    libgcc-s1 \
    libssl-dev \
    pkg-config \
    clang \
    libclang-dev \
    zlib1g-dev \
    llvm-dev \
    build-essential && \
    rm -rf /var/lib/apt/lists/*


WORKDIR /app

RUN cargo install cargo-watch

COPY . .

WORKDIR /app/server

RUN cargo build

VOLUME ["/app", "/cargo-target", "/usr/local/cargo/registry"]

CMD ["cargo", "watch", "-qcx",  "run --bin orchard" ]

EXPOSE 8000