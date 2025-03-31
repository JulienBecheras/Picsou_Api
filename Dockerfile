# Utiliser l'image Alpine de Rust
FROM rust:latest

# Installer les dépendances système nécessaires
RUN apt-get update && apt-get install -y git libpq-dev

WORKDIR /app

# Récuperation du projet
COPY . .

# Récuperer les dépendances
RUN cargo install diesel_cli --no-default-features --features postgres

# Compiler le projet
RUN cargo build --release

# Créer un fichier d'entrée pour le conteneur
RUN chmod +x entrypoint.sh
CMD ["./entrypoint.sh"]

# Exposer le port (ajuste le port selon ton API)
EXPOSE 8000
