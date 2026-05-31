# 🚀 File Upload Service (Rust + Axum)

Лёгкий сервис для загрузки и скачивания файлов с авторизацией через JWT.

## ✨ Основные возможности

* 📤 Загрузка файлов с авторизацией (`JWT`)
* 📥 Скачивание файлов по URL
* 🔐 Генерация upload-токенов через internal endpoint
* 🧹 Автоматическое ограничение количества файлов (`MAX_FILES`)
* ✅ Валидация имён файлов (защита от path traversal)
* 📡 Готов к интеграции с API Gateway / Elastic / Observability

***

# 🏗 Архитектура

```
/upload          -> загрузка файла (JWT required)
/download/{file} -> скачивание файла
/internal/token  -> генерация JWT (internal access)
```

***

# ⚙️ Конфигурация

Через переменные окружения:

| Переменная        | Описание                      | По умолчанию            |
| ----------------- | ----------------------------- | ----------------------- |
| `PORT`            | Порт сервиса                  | 3000                    |
| `UPLOAD_DIR`      | Директория хранения файлов    | ./data                  |
| `JWT_SECRET`      | Секрет для подписи JWT        | dev\_secret\_change\_me |
| `INTERNAL_SECRET` | Секрет для internal API       | internal\_dev\_secret   |
| `PUBLIC_BASE_URL` | Базовый URL для ссылок        | auto                    |
| `URI_PREFIX`      | Префикс API (например `/api`) | пусто                   |

***

# 🔐 JWT

## Claims

```json
{
  "sub": "uploader-service",
  "scope": "upload",
  "iat": 1717000000,
  "exp": 1719600000
}
```

## TTL

Конфигурируется в:

```rust
pub const JWT_LIFESPAN: Duration = Duration::days(30);
```

***

# 🔑 Получить токен

```bash
curl -s \
  -H "x-internal-secret: internal_dev_secret" \
  "http://localhost:3000/internal/token?sub=uploader-service"
```

***

# 📤 Загрузка файла

```bash
curl -s \
  -H "Authorization: Bearer $TOKEN" \
  -F "file=@./report.xlsx;filename=report.xlsx" \
  http://localhost:3000/upload
```

Ответ:

```json
{
  "file": "report.xlsx",
  "download_url": "http://localhost:3000/download/report.xlsx"
}
```

***

# 📥 Скачать файл

```bash
curl -OJ http://localhost:3000/download/report.xlsx
```

***

# 🧹 Ограничение файлов

* Максимум файлов: `MAX_FILES`
* Старые файлы автоматически удаляются

```rust
pub const MAX_FILES: usize = 5;
```

***

# 🛡 Безопасность

## ✅ Уже реализовано

* Валидация имени файла:
    * запрещены `/`, `\`, `..`
* JWT scope (`upload`)
* internal endpoint защищён секретом

# 📊 Логирование

Сервис использует `tracing`.

## Запуск с логами

```bash
RUST_LOG=info cargo run
```

## Форматы логов

### Для разработки:

```rust
.pretty()
```

### Для production:

```rust
.json()
```

***

# 🔍 Пример логов

```json
{
  "level": "INFO",
  "action": "jwt_create",
  "sub": "uploader-service",
  "ttl_seconds": 2592000
}
```

***

# 🧩 Request ID (observability)

Каждый запрос получает уникальный `request_id`.

Позволяет:

* отслеживать цепочку вызовов
* удобно дебажить через Elastic

***

# 🏥 Healthcheck

```bash
GET /health
```

Ответ:

```json
{
  "status": "ok"
}
```

***

# 🚀 Запуск

```bash
cargo run
```

или:

```bash
RUST_LOG=info cargo run
```

***

# 📦 Структура проекта

```
src/
 ├── app.rs           # роутинг и middleware
 ├── main.rs          # точка входа
 ├── state.rs         # состояние приложения
 ├── config.rs        # конфигурация
 ├── auth/
 │    └── jwt.rs      # JWT логика
 ├── routes/
 │    ├── upload.rs
 │    ├── download.rs
 │    ├── internal.rs
 │    └── health.rs
 ├── storage/
 │    └── file_manager.rs
 ├── utils/
 │    ├── sanitize.rs
 │    └── uri.rs
 └── error.rs
```

***

# 🧠 Known limitations

* нет ограничения размера файла
* нет revoke токенов
* нет refresh токенов
* простой internal auth (header-based)

***

