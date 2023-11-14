FROM rust:alpine as builder

WORKDIR /app

COPY . /app

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/release/coub_saver_bot /usr/local/bin/coub_saver_bot

RUN apk --no-cache add ca-certificates \
    && rm -rf /var/cache/apk/*

CMD ["coub_saver_bot"]