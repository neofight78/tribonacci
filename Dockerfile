FROM rust:1.56.0 AS build

RUN apt update && apt install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/tribonacci .

ENV APP_ENVIRONMENT production

CMD ["./tribonacci"]
