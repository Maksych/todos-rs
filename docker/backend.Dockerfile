FROM rust:alpine as builder

RUN apk update \
    && apk add pkgconfig musl-dev openssl-dev

WORKDIR /app

COPY backend .

RUN cargo build --release


FROM alpine

RUN apk update \
    && apk add openssl-dev

COPY --from=builder /app/target/release/backend /

ENTRYPOINT [ "/backend" ]
