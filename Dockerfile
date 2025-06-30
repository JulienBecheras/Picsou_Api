# Étape 1 : build de l'application
FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y libpq-dev pkg-config

# ⬇️ Installer diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release

# Étape 2 : image minimale
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/projet_picsou_api /usr/local/bin/api
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV RUST_BACKTRACE=1

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
