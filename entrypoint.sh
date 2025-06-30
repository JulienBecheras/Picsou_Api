#!/bin/sh
set -e

cd /usr/local/bin/

echo "▶️ Migration de la base de données..."
diesel migration run --database-url "$DATABASE_URL"

echo "🚀 Lancement de l'API..."
exec ./target/release/projet_picsou_api
