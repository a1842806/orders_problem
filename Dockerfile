FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

RUN rm -rf src

COPY . .

RUN cargo build --release

FROM rust:slim

COPY --from=builder /usr/src/app/target/release/orders_problem /usr/local/bin/orders_problem

CMD ["orders_problem"]