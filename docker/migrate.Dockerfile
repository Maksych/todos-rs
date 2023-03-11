FROM rust:alpine as builder

RUN apk update \
    && apk add pkgconfig musl-dev openssl-dev

COPY .cargo .cargo

RUN cargo install sqlx-cli


FROM alpine

COPY --from=builder  usr/local/cargo/bin/sqlx /

COPY backend/migrations migrations

CMD /sqlx database create \
    && /sqlx migrate run
