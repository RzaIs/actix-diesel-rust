# Build image
FROM rust:latest as build

WORKDIR /build

COPY ./ /build

RUN cargo install --path .

# Runtime image
FROM debian:buster-slim

COPY --from=build /usr/local/cargo/bin/ms-sample /usr/local/bin/application

RUN apt update
RUN apt install -y libpq5 libpq-dev

CMD ["application"]
