FROM clux/muslrust:stable AS builder
RUN apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates
# FROM rust:latest AS builder
# RUN rustup target add x86_64-unknown-linux-musl
# RUN apt -y update
# RUN apt install -y musl-tools musl-dev
# RUN apt-get install -y build-essential
# RUN apt install -y gcc-x86-64-linux-gnu

WORKDIR /app
COPY ./ .

# ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/warp-docker ./
CMD ["/app/warp-docker"]
