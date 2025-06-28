# Use Alpine Linux as base image for smaller size
FROM alpine:latest

# Install necessary packages
RUN apk add --no-cache ca-certificates tzdata

# Create a non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -s /bin/sh -u 1000 -G appuser appuser

# Set working directory
WORKDIR /app

# Copy the binary (path will be determined by build context)
ARG TARGETARCH
COPY ${TARGETARCH}/cf-ddns-rust /app/cf-ddns-rust

# Make binary executable and set ownership
RUN chmod +x /app/cf-ddns-rust && \
    chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Set entrypoint
ENTRYPOINT ["/app/cf-ddns-rust"]