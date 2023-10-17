FROM rust:latest as builder

WORKDIR /app/

COPY . .

RUN rustup default

RUN cargo build --release 
RUN rm -rf ./src


FROM rust:latest as runner

COPY --from=builder /app/target/release/server /usr/local/bin/rust_template_server

WORKDIR /usr/local/bin

ENV ROCKET_PROFILE=release ROCKET_ADDRESS=0.0.0.0
CMD ["rust_template_server"]
