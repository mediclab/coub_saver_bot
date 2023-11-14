FROM rust:alpine as builder

ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /app

COPY . /app

RUN apk --no-cache add pkgconfig musl-dev openssl-dev clang-dev build-base make \
    && rm -rf /var/cache/apk/*

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/release/coub_saver_bot /usr/local/bin/coub_saver_bot

RUN apk --no-cache add ca-certificates openssl libgcc libstdc++ \
    && rm -rf /var/cache/apk/*

CMD ["coub_saver_bot"]