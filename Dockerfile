# Use Alpine Linux as base image for smaller size
FROM alpine:latest

# Install necessary packages
RUN apk add --no-cache ca-certificates tzdata

# Create a non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -s /bin/sh -u 1000 -G appuser appuser

# Set working directory
WORKDIR /app

# Copy the binary from host (will be built in CI)
COPY cf-ddns-rust /app/cf-ddns-rust

# Make binary executable
RUN chmod +x /app/cf-ddns-rust

# Change ownership to appuser
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port if needed (adjust as necessary)
EXPOSE 8080

# Set entrypoint
ENTRYPOINT ["/app/cf-ddns-rust"]