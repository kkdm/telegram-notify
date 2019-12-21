FROM clux/muslrust:nightly-2019-12-09 as telegram-notify-builder

ADD . /src
WORKDIR /src

RUN cargo build --release --target x86_64-unknown-linux-musl --target-dir /usr/local

FROM alpine

COPY --from=telegram-notify-builder /usr/local/x86_64-unknown-linux-musl/release/telegram-notify /usr/local/bin/
