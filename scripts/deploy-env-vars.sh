#!/usr/bin/env bash
set -euo pipefail

# --------------------------------------------------
# Configuración
# --------------------------------------------------
RESOURCE_GROUP="lemipay-backend-rg"
APP_NAME="lemipay-backend-api"
ENV_FILE=".env.azure"

if [ ! -f "$ENV_FILE" ]; then
    echo "❌ No existe $ENV_FILE"
    exit 1
fi

echo "📖 Leyendo $ENV_FILE..."

SECRETS=()
ENV_VARS=()

while IFS='=' read -r KEY VALUE || [ -n "$KEY" ]; do
    # Ignorar comentarios y líneas vacías
    [[ -z "$KEY" ]] && continue
    [[ "$KEY" =~ ^# ]] && continue

    # Eliminar espacios
    KEY=$(echo "$KEY" | xargs)

    # Todo lo que viene después del primer '=' pertenece al valor
    VALUE="${VALUE:-}"

    # nombre secreto -> lowercase + guiones
    SECRET_NAME=$(echo "$KEY" | tr '[:upper:]' '[:lower:]' | tr '_' '-')

    SECRETS+=("${SECRET_NAME}=${VALUE}")
    ENV_VARS+=("${KEY}=secretref:${SECRET_NAME}")

done < "$ENV_FILE"

echo "🔐 Actualizando secretos..."

az containerapp secret set \
    --resource-group "$RESOURCE_GROUP" \
    --name "$APP_NAME" \
    --secrets "${SECRETS[@]}"

echo "🌎 Actualizando variables de entorno..."

az containerapp update \
    --resource-group "$RESOURCE_GROUP" \
    --name "$APP_NAME" \
    --set-env-vars "${ENV_VARS[@]}"

echo
echo "✅ Variables desplegadas correctamente."

echo
echo "⏳ Esperando que se cree la nueva revisión..."

sleep 10

az containerapp revision list \
  -g "$RESOURCE_GROUP" \
  -n "$APP_NAME" \
  -o table