FROM rust:1-alpine AS build

COPY . /app
WORKDIR /app

RUN cargo build --bin sidecar --release --locked
RUN cargo install --path . --locked

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/sidecar /usr/local/bin/sidecar

CMD ["sidecar"]