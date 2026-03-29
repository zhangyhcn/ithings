FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libzmq3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY zmq-router /app/zmq-router

EXPOSE 5550 5551

ENTRYPOINT ["/app/zmq-router"]
CMD ["--frontend", "tcp://*:5550", "--backend", "tcp://*:5551", "--loglevel", "info"]
