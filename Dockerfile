# --- Build Stage ---
FROM rust:1.75-alpine as builder

RUN apk add --no-cache musl-dev gcc

WORKDIR /app
COPY . .

# Build the release binary
RUN cargo build --release

# --- Runtime Stage ---
FROM alpine:3.18

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/dirbrute /usr/local/bin/dirbrute
# Copy the UI directory
COPY --from=builder /app/ui ./ui

# Expose the default Web UI port
EXPOSE 8080

# Default command starts the web server
ENTRYPOINT ["dirbrute"]
CMD ["web", "--port", "8080"]
