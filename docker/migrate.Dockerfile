FROM rust:slim-buster

RUN apt update \
    && apt -y install pkg-config libssl-dev

RUN cargo install sqlx-cli

WORKDIR /app

COPY ./migrations ./migrations

CMD sqlx database create \
    && sqlx migrate run
