FROM rust:slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev protobuf-compiler
RUN yes | apt install -y gcc-x86-64-linux-gnu
RUN update-ca-certificates

WORKDIR /app
COPY . .

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch as gateway
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/gateway /gateway
CMD [ "/gateway" ]
LABEL service=gateway

FROM scratch as service1
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/service1 /service1
CMD [ "/service1" ]
LABEL service=service1

FROM scratch as service2
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/service2 /service2
CMD [ "/service2" ]
LABEL service=service2