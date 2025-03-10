# rust version 1.85 alpine image
FROM 1.85-alpine3.21
# set work directory and copy source to it
WORKDIR /app
COPY ./ /app
# build release version
RUN cargo build --release

# use alpine image
FROM alpine3.21
# copy binary file
COPY --from=0 /app/target/release/rust-axum-todo-list .
# set the binary as entrypoint
ENTRYPOINT ["/rust-axum-todo-list"]
