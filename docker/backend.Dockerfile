FROM rust:slim-buster as build

RUN apt update \
    && apt -y install pkg-config libssl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt update \
    && apt -y install libssl-dev

COPY --from=build /app/target/release/backend /

ENTRYPOINT [ "/backend" ]
