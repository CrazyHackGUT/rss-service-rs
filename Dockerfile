# Build project
FROM rust:1.64-alpine AS build
COPY ./ ./
RUN apk add --no-cache clang musl-dev openssl-dev postgresql14-dev
RUN cargo build --release

# Create an empty image
FROM rust:1.64-alpine
WORKDIR /app

COPY --from=build ./target/release/rss-service /app
ENTRYPOINT ["/app/rss-service"]
