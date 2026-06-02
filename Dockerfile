# ---------- Build stage ----------
FROM rust:1.96-alpine AS builder

WORKDIR /app

# Копируем файлы зависимостей
COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src

COPY src /app/src

# Финальная сборка реального приложения
RUN cargo build --release

# ---------- Runtime stage ----------
FROM alpine:latest

WORKDIR /app

# Устанавливаем зависимости
RUN apk add --no-cache ca-certificates

# Копируем бинарник
COPY --from=builder /app/target/release/simple-file-storage /usr/local/bin/simple-file-storage

RUN mkdir -p /data

ENV RUST_LOG=info
ENV PORT=3000
ENV URI_PREFIX=
ENV UPLOAD_DIR=/data

EXPOSE 3000

CMD ["simple-file-storage"]
