# Étape 1 : build de l'application
FROM rust:1.77 as builder

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
COPY --from=builder /app/target/release/mon-api /usr/local/bin/api

# Expose le port utilisé par ton API
EXPOSE 8000

# Lancement de l’API directement (pas de wait-for-it.sh)
CMD ["api"]
