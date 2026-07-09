FROM rust:1.56

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

CMD ["./target/release/backend"]