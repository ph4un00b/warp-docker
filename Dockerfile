FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev

WORKDIR /app
COPY ./ .

ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/warp-docker ./
CMD ["/app/warp-docker"]
