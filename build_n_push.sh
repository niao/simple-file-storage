#!/bin/bash
# Build and push Docker image
# По умолчанию — значение из оригинального скрипта
DEFAULT_REGISTRY="localhost:5000"

# Приоритет заполнения REGISTRY:
# 1. Параметр командной строки (первый аргумент)
# 2. Переменная окружения REGISTRY
# 3. Значение по умолчанию
if [ -n "$1" ]; then
    REGISTRY="$1"
elif [ -n "$REGISTRY" ]; then
    # Переменная окружения уже установлена, используем её
    :
else
    REGISTRY="$DEFAULT_REGISTRY"
fi

# Извлекаем имя пакета из cargo.toml
IMAGE_NAME=$(grep '^name =' Cargo.toml | sed -E 's/name = "([^"]+)"/\1/')

# Извлекаем версию из cargo.toml
VERSION=$(grep '^version =' Cargo.toml | sed -E 's/version = "([^"]+)"/\1/')

docker build -t $REGISTRY/$IMAGE_NAME:$VERSION -f Dockerfile .
docker push $REGISTRY/$IMAGE_NAME:$VERSION
echo "Docker image pushed successfully."
