FROM rust:1.56.1 AS builder

WORKDIR /builder

COPY server/Cargo.toml .
COPY Cargo.lock .
RUN set -x\
 && mkdir -p src\
 && echo "fn main() {}" > src/main.rs\
 && cargo build --release

COPY server/src src
#COPY sqlx-data.json .
#ENV SQLX_OFFLINE true
RUN set -x\
 && find target/release/ -type f -executable -maxdepth 1 -delete\
 && cargo build --release

FROM rust:1.56.1-slim
COPY --from=builder /builder/target/release/server /usr/local/bin
COPY server/Configuration.toml /usr/local/bin

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin"]
CMD ["./server"]


