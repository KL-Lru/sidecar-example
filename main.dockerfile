FROM rust:1-alpine AS build

COPY . /app
WORKDIR /app

RUN cargo build --bin main --release --locked
RUN cargo install --path . --locked

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/main /usr/local/bin/main

CMD ["main"]