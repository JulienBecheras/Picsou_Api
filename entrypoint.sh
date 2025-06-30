#!/bin/sh
set -e

echo "▶️ Migration de la base de données..."
diesel migration run --database-url "$DATABASE_URL"

echo "🚀 Lancement de l’API..."
exec /usr/local/bin/api
