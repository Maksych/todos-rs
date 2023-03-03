FROM rust:alpine as builder

RUN apk update \
    && apk add pkgconfig musl-dev openssl-dev

WORKDIR /app

COPY backend .

RUN cargo build --release


FROM alpine

COPY --from=builder /app/target/release/backend /

ENTRYPOINT [ "/backend" ]
