# 📁 Структура Helm chart

```
file-upload-chart/
 ├── Chart.yaml
 ├── values.yaml
 └── templates/
     ├── deployment.yaml
     ├── service.yaml
     ├── ingress.yaml
     ├── pvc.yaml
     ├── secret.yaml
     └── _helpers.tpl
```

***

# 📦 Chart.yaml

```yaml
apiVersion: v2
name: file-upload
description: File upload service (Rust + Axum)
version: 0.1.0
appVersion: "1.0.0"
```

***

# ⚙️ values.yaml

```yaml
replicaCount: 2

image:
  repository: your-registry/file-upload
  tag: latest
  pullPolicy: IfNotPresent

service:
  type: ClusterIP
  port: 80

containerPort: 3000

env:
  RUST_LOG: info
  PORT: "3000"
  UPLOAD_DIR: "/data"

secrets:
  jwtSecret: "super-secret"
  internalSecret: "internal-secret"

persistence:
  enabled: true
  size: 5Gi

ingress:
  enabled: true
  host: upload.example.com

resources:
  limits:
    cpu: "500m"
    memory: "256Mi"
  requests:
    cpu: "100m"
    memory: "128Mi"
```

***

# 🧠 templates/\_helpers.tpl

```yaml
{{- define "file-upload.name" -}}
file-upload
{{- end }}

{{- define "file-upload.fullname" -}}
{{ .Release.Name }}-file-upload
{{- end }}
```

***

# 🚀 templates/deployment.yaml

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ include "file-upload.fullname" . }}
spec:
  replicas: {{ .Values.replicaCount }}
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
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag }}"
          imagePullPolicy: {{ .Values.image.pullPolicy }}

          ports:
            - containerPort: {{ .Values.containerPort }}

          env:
            - name: PORT
              value: "{{ .Values.env.PORT }}"
            - name: UPLOAD_DIR
              value: "{{ .Values.env.UPLOAD_DIR }}"
            - name: RUST_LOG
              value: "{{ .Values.env.RUST_LOG }}"

            - name: JWT_SECRET
              valueFrom:
                secretKeyRef:
                  name: {{ include "file-upload.fullname" . }}
                  key: jwt-secret

            - name: INTERNAL_SECRET
              valueFrom:
                secretKeyRef:
                  name: {{ include "file-upload.fullname" . }}
                  key: internal-secret

          volumeMounts:
            - name: storage
              mountPath: /data

          readinessProbe:
            httpGet:
              path: /health
              port: {{ .Values.containerPort }}
            initialDelaySeconds: 5
            periodSeconds: 5

          livenessProbe:
            httpGet:
              path: /health
              port: {{ .Values.containerPort }}
            initialDelaySeconds: 10
            periodSeconds: 10

          resources:
{{ toYaml .Values.resources | indent 12 }}

      volumes:
        - name: storage
          persistentVolumeClaim:
            claimName: {{ include "file-upload.fullname" . }}
```

***

# 🌐 templates/service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: {{ include "file-upload.fullname" . }}
spec:
  type: {{ .Values.service.type }}
  selector:
    app: file-upload
  ports:
    - port: {{ .Values.service.port }}
      targetPort: {{ .Values.containerPort }}
```

***

# 🌍 templates/ingress.yaml

```yaml
{{- if .Values.ingress.enabled }}
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: {{ include "file-upload.fullname" . }}
spec:
  rules:
    - host: {{ .Values.ingress.host }}
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: {{ include "file-upload.fullname" . }}
                port:
                  number: {{ .Values.service.port }}
{{- end }}
```

***

# 💾 templates/pvc.yaml

```yaml
{{- if .Values.persistence.enabled }}
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: {{ include "file-upload.fullname" . }}
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: {{ .Values.persistence.size }}
{{- end }}
```

***

# 🔐 templates/secret.yaml

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: {{ include "file-upload.fullname" . }}
type: Opaque
stringData:
  jwt-secret: "{{ .Values.secrets.jwtSecret }}"
  internal-secret: "{{ .Values.secrets.internalSecret }}"
```

***

# 🚀 Установка

```bash
helm install file-upload ./file-upload-chart
```

***

# 🔄 Обновление

```bash
helm upgrade file-upload ./file-upload-chart
```

***

# ❌ Удаление

```bash
helm uninstall file-upload
```

***

# ⚡ Пример кастомизации

```bash
helm install file-upload ./file-upload-chart \
  --set image.repository=myrepo/file-upload \
  --set ingress.host=upload.dev.company.com
```

***

