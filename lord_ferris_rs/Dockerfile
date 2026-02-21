FROM rust:1.93-alpine AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src && echo '' > src/lib.rs

RUN cargo fetch

# Remove the temporary file before copying the actual source code
RUN rm -rf src

# Copy the actual source code separately
COPY src ./src

RUN cargo build --release

# Stage 2: Runtime Stage
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/lord_ferris_rs ./lord_ferris_rs

CMD ["/app/lord_ferris_rs"]
