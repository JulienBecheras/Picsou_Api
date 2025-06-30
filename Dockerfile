# Étape 1 : build de l'application
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Copie des fichiers du projet
COPY . .

# Installer les dépendances
RUN apt-get update && apt-get install -y libpq-dev pkg-config libssl-dev

# Installer Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Appliquer les migrations (en supposant que DATABASE_URL est défini au build, sinon à faire plus tard)
# COMMENTÉ POUR RAILWAY, car Railway fournit DATABASE_URL à l'exécution
# RUN diesel migration run --database-url $DATABASE_URL

# Compilation du projet
RUN cargo build --release

# Étape 2 : image finale minimale
FROM debian:bookworm-slim

WORKDIR /app

# Installer les dépendances nécessaires
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copier le binaire compilé
COPY --from=builder /app/target/release/projet_picsou_api /usr/local/bin/api
# Copier le binaire diesel depuis le builder
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copier les migrations
COPY ./migrations ./migrations
COPY ./diesel.toml .

# Configuration Rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

# Expose le port utilisé par ton API
EXPOSE 8080

# Entrypoint et démarrage
COPY entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["entrypoint.sh"]
