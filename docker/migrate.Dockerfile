FROM rust:alpine as builder

RUN apk update \
    && apk add pkgconfig musl-dev openssl-dev

RUN cargo install sqlx-cli


FROM alpine

RUN apk update \
    && apk add openssl-dev

COPY --from=builder  usr/local/cargo/bin/sqlx /

COPY backend/migrations migrations

CMD /sqlx database create \
    && /sqlx migrate run
