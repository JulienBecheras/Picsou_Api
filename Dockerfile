# Utiliser l'image Alpine de Rust
FROM rust:latest

# Récuperation du projet
COPY . .
WORKDIR projet_picsou_api

# Récuperation des variables d'environnement
COPY ./.env .

# Récuperer les dépendances
RUN cargo fetch
RUN cargo install diesel_cli --no-default-features --features postgres
RUN echo DATABASE_URL=postgres://admin:i0h70JmTynW9@db:5432/postgres > .env

# Effectuer la configuration de la base de données et les migrations
RUN diesel setup
RUN diesel migration run

# Compiler le projet
RUN cargo build --release

# Exposer le port (ajuste le port selon ton API)
EXPOSE 8000

# Démarrer l'application
CMD ["cargo", "run", "--release"]
