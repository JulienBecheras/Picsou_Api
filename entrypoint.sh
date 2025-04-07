#!/bin/sh
set -e

echo "Attente de la base de données..."
./wait-for-it.sh db:5432 --timeout=30 --strict -- echo "La base de données est prête !"

echo "preparation de diesel..."
diesel setup

echo "Appliquer les migrations..."
diesel migration run

echo "Compilation du projet..."
cargo build --release

echo "Lancement de l'API..."
exec ./target/release/projet_picsou_api
