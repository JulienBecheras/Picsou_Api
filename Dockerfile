# Étape 1 : build de l'application
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Copie des fichiers du projet
COPY . .

# Installer libpq-dev pour diesel_postgres
RUN apt-get update && apt-get install -y libpq-dev pkg-config

# Compilation en release
RUN cargo build --release

# Étape 2 : image finale minimale
FROM debian:bookworm-slim

# Installer les dépendances nécessaires à l'exécution
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copier le binaire compilé
COPY --from=builder /app/target/release/projet_picsou_api /usr/local/bin/api

# Copier le script d'initialisation
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Configuration Rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

# Variables d'environnement pour Diesel (Railway passera DATABASE_URL automatiquement)
ENV RUST_BACKTRACE=1

# Expose le port utilisé par ton API
EXPOSE 8080

# Utilisation du script comme point d’entrée
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
