Конечно — давай сделаем уже **production‑ready README с деплоем в Kubernetes**, прямо пригодный для твоего текущего сервиса 🚀

***

# 🚀 File Upload Service (Rust + Axum)

Сервис для загрузки и скачивания файлов с авторизацией через JWT, готовый к деплою в Kubernetes.

***

# ✨ Возможности

* 📤 Upload файлов через `/upload` (JWT required)
* 📥 Download файлов через `/download/{filename}`
* 🔐 Генерация JWT через `/internal/token`
* 🧹 Авто‑очистка файлов (`MAX_FILES`)
* 📊 Structured logging (`tracing`)
* 🧩 Request ID для observability
* ☸️ Полная поддержка Kubernetes

***

# 🏗 Архитектура

```
Client
   ↓
API Gateway / Ingress
   ↓
Service (Kubernetes)
   ↓
Pod (Rust + Axum)
   ↓
Storage (volume / PVC)
```

***

# ⚙️ Переменные окружения

| Переменная        | Описание            | Пример                    |
| ----------------- | ------------------- | ------------------------- |
| `PORT`            | порт сервиса        | 3000                      |
| `UPLOAD_DIR`      | директория файлов   | /data                     |
| `JWT_SECRET`      | секрет JWT          | strong-secret             |
| `INTERNAL_SECRET` | internal API секрет | internal-secret           |
| `PUBLIC_BASE_URL` | публичный URL       | <https://api.company.com> |
| `RUST_LOG`        | уровень логов       | info                      |

***

# 🐳 Docker

## Dockerfile

```dockerfile
FROM rust:1.76 as builder

WORKDIR /app
COPY .. .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/app /app/app

RUN mkdir -p /data

ENV PORT=3000
ENV UPLOAD_DIR=/data

CMD ["./app"]
```

***

# ☸️ Kubernetes Deployment

## 📦 Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: file-upload
spec:
  replicas: 2
  selector:
    matchLabels:
      app: file-upload
  template:
    metadata:
      labels:
        app: file-upload
    spec:
      containers:
        - name: app
          image: your-registry/file-upload:latest
          ports:
            - containerPort: 3000

          env:
            - name: PORT
              value: "3000"
            - name: UPLOAD_DIR
              value: "/data"
            - name: RUST_LOG
              value: "info"
            - name: JWT_SECRET
              valueFrom:
                secretKeyRef:
                  name: file-upload-secrets
                  key: jwt-secret
            - name: INTERNAL_SECRET
              valueFrom:
                secretKeyRef:
                  name: file-upload-secrets
                  key: internal-secret

          volumeMounts:
            - name: storage
              mountPath: /data

          readinessProbe:
            httpGet:
              path: /health
              port: 3000
            initialDelaySeconds: 5
            periodSeconds: 5

          livenessProbe:
            httpGet:
              path: /health
              port: 3000
            initialDelaySeconds: 10
            periodSeconds: 10

      volumes:
        - name: storage
          persistentVolumeClaim:
            claimName: file-upload-pvc
```

***

## 💾 PersistentVolumeClaim

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: file-upload-pvc
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 5Gi
```

***

## 🔐 Secrets

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: file-upload-secrets
type: Opaque
stringData:
  jwt-secret: "super-strong-secret"
  internal-secret: "internal-secret"
```

***

## 🌐 Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: file-upload
spec:
  selector:
    app: file-upload
  ports:
    - port: 80
      targetPort: 3000
  type: ClusterIP
```

***

## 🌍 Ingress (пример)

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: file-upload
spec:
  rules:
    - host: upload.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: file-upload
                port:
                  number: 80
```

***

# 🚀 Деплой

```bash
kubectl apply -f secrets.yaml
kubectl apply -f pvc.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
kubectl apply -f ingress.yaml
```

***

# 🔑 Использование

## Получить токен

```bash
curl -H "x-internal-secret: internal-secret" \
  "https://upload.example.com/internal/token?sub=test"
```

***

## Загрузка файла

```bash
curl -H "Authorization: Bearer TOKEN" \
  -F "file=@test.txt" \
  https://upload.example.com/upload
```

***

## Скачать

```bash
curl -OJ https://upload.example.com/download/test.txt
```

***

# 📊 Логирование

Включено через `tracing`.

## Запуск

```bash
RUST_LOG=info
```

## Режимы

* dev → pretty
* prod → json (для Elastic)

***

# ⚠️ Ограничения текущей версии

* нет ограничения размера файла
* нет масштабируемого storage
* нет revoke токенов
* download публичный

***


