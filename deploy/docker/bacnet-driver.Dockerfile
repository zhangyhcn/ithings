# Runtime stage - uses pre-built binary from local compilation
# Local rustc 1.94 already built the binary, we just copy it
FROM debian:bookworm-slim

WORKDIR /app

# Use Chinese mirrors for better download speed in mainland China
RUN rm -f /etc/apt/sources.list \
 && printf "deb http://mirrors.tuna.tsinghua.edu.cn/debian/ bookworm main contrib non-free\n\
deb http://mirrors.tuna.tsinghua.edu.cn/debian/ bookworm-updates main contrib non-free\n\
deb http://mirrors.tuna.tsinghua.edu.cn/debian-security bookworm-security main contrib non-free\n" > /etc/apt/sources.list

# Install runtime dependencies and debug tools
RUN apt-get update \
 && apt-get install -y --fix-missing \
    libzmq3-dev \
    ca-certificates \
    iputils-ping \
    curl \
    net-tools \
 && rm -rf /var/lib/apt/lists/*

COPY driver-bacnet /usr/local/bin/

# Create non-root user
RUN useradd -m appuser
USER appuser

# Expose port
EXPOSE 47808

# Entry point
ENTRYPOINT ["driver-bacnet"]
