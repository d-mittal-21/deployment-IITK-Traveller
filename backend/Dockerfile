FROM rust:latest as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src/ ./src/

RUN cargo build --release

FROM ubuntu:20.04

# RUN apk --no-cache add ca-certificates openssl
RUN apt-get update
RUN apt-get install -y ca-certificates
RUN apt-get install -y openssl


COPY --from=builder /usr/src/app/target/release/backend .

EXPOSE 8080

CMD ["./backend"]