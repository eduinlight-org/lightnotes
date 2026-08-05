FROM debian:trixie-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/dx/landing/release/web/server /app/server
COPY target/dx/landing/release/web/public /app/public

ENV IP=0.0.0.0 \
    PORT=8080

EXPOSE 8080

CMD ["/app/server"]
