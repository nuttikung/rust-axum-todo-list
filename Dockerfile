# rust version 1.85 alpine image
FROM rust:1.85-alpine3.21 AS builder
# add dependencies
RUN apk add --no-cache pkgconfig
RUN apk add --no-cache musl-dev
RUN apk add --no-cache libressl-dev
# set work directory and copy source to it
WORKDIR /app
COPY ./ /app
RUN ls -ltr Setting.toml
# build release version
# RUN cargo build --release

# runtime stage
# FROM alpine:3.21 AS runtime
# # copy binary file
# COPY --from=builder /app/target/release/rust-axum-todo-list .
# # set the binary as entrypoint
# ENTRYPOINT ["/rust-axum-todo-list"]
