# ---------- Build stage ----------
FROM rust:1.96 AS builder

WORKDIR /app

# Копируем файлы зависимостей
COPY Cargo.toml Cargo.lock ./

# Создаём заглушку для кэширования зависимостей
RUN mkdir src \
    && echo 'fn main() { println!("Hello, world!"); }' > src/main.rs

# Собираем зависимости (кэшируется при неизменных Cargo.toml/Cargo.lock)
RUN cargo build --release

# Удаляем заглушку
RUN rm -rf src

# Копируем исходный код
COPY src ./src

# Финальная сборка приложения
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

# Устанавливаем необходимые системные зависимости
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Копируем собранный бинарник (используем правильное имя из Cargo.toml)
COPY --from=builder /app/target/release/simple-file-storage /usr/local/bin/simple-file-storage

# Создаём директорию для данных
RUN mkdir -p /data

# Задаём переменные окружения
ENV RUST_LOG=info
ENV PORT=3000
ENV URI_PREFIX=
ENV UPLOAD_DIR=/data

# Открываем порт
EXPOSE 3000

# Команда запуска (используем правильное имя бинарника)
CMD ["simple-file-storage"]
