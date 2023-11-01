FROM rust:latest as builder

WORKDIR /app/

COPY . .

RUN rustup default

RUN cargo build --release 
RUN rm -rf ./src


FROM rust:latest as runner

RUN mkdir /opt/template_api

COPY --from=builder /app/target/release/server /opt/template_api/api_server
COPY --from=builder /app/templates /opt/template_api/templates

WORKDIR /opt/template_api

ENV ROCKET_PROFILE=release ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=433

CMD ./api_server