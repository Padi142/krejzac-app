FROM rust:1.66-slim-buster

RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

EXPOSE 8080

CMD ["trunk","serve","--release","--address","0.0.0.0"]
