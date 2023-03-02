FROM node:alpine as css_builder

WORKDIR /app

COPY frontend/package.json package.json

RUN npm install

COPY frontend .

RUN npx tailwindcss -i input.css -o output.css --minify


FROM rust:alpine as builder

RUN apk update \
    && apk add pkgconfig musl-dev openssl-dev \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk wasm-bindgen-cli

WORKDIR /app

COPY frontend .

COPY --from=css_builder /app/output.css output.css

ARG BASE_URL

ENV BASE_URL ${BASE_URL}

RUN trunk build --release


FROM nginx:alpine-slim

WORKDIR /app

COPY --from=builder /app/dist /app

COPY docker/frontend.nginx.conf /etc/nginx/conf.d/default.conf
