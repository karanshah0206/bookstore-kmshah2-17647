#!/usr/bin/env bash
set -euo pipefail

# Usage: ./deployment/scripts/init_aws_database.sh <stack-name> <db-username> <db-password> [namespace]
STACK_NAME="${1:-}"
DB_USERNAME="${2:-}"
DB_PASSWORD="${3:-}"
NAMESPACE="${4:-bookstore-ns}"

if [[ -z "$STACK_NAME" || -z "$DB_USERNAME" || -z "$DB_PASSWORD" ]]; then
  echo "Usage: $0 <stack-name> <db-username> <db-password> [namespace]"
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEPLOYMENT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

"$SCRIPT_DIR/sync_db_endpoint_from_cfn.sh" "$STACK_NAME" "$NAMESPACE"

kubectl create configmap bookstore-db-sql \
  --namespace "$NAMESPACE" \
  --from-file=init_books_db.sql="$DEPLOYMENT_DIR/init_books_db.sql" \
  --from-file=init_customer_db.sql="$DEPLOYMENT_DIR/init_customer_db.sql" \
  --dry-run=client -o yaml | kubectl apply -f -

kubectl create secret generic bookstore-db-credentials \
  --namespace "$NAMESPACE" \
  --from-literal=DATABASE_USERNAME="$DB_USERNAME" \
  --from-literal=DATABASE_PASSWORD="$DB_PASSWORD" \
  --dry-run=client -o yaml | kubectl apply -f -

cat <<'YAML' | kubectl apply -f -
apiVersion: batch/v1
kind: Job
metadata:
  name: bookstore-db-init
  namespace: bookstore-ns
spec:
  backoffLimit: 1
  template:
    metadata:
      labels:
        app: bookstore-db-init
    spec:
      restartPolicy: Never
      containers:
        - name: mysql-client
          image: mysql:8.0
          imagePullPolicy: Always
          env:
            - name: DATABASE_ENDPOINT
              valueFrom:
                configMapKeyRef:
                  name: bookstore-db-endpoints
                  key: aurora_writer_endpoint
            - name: DATABASE_USERNAME
              valueFrom:
                secretKeyRef:
                  name: bookstore-db-credentials
                  key: DATABASE_USERNAME
            - name: DATABASE_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: bookstore-db-credentials
                  key: DATABASE_PASSWORD
          volumeMounts:
            - name: sql-files
              mountPath: /sql
          command:
            - sh
            - -ceu
            - |
              DB_HOST="${DATABASE_ENDPOINT%:*}"
              DB_PORT="${DATABASE_ENDPOINT##*:}"

              until mysqladmin ping \
                -h "$DB_HOST" \
                -P "$DB_PORT" \
                -u "$DATABASE_USERNAME" \
                -p"$DATABASE_PASSWORD" \
                --silent; do
                echo "Waiting for Aurora to accept connections..."
                sleep 10
              done

              mysql \
                -h "$DB_HOST" \
                -P "$DB_PORT" \
                -u "$DATABASE_USERNAME" \
                -p"$DATABASE_PASSWORD" \
                < /sql/init_books_db.sql

              mysql \
                -h "$DB_HOST" \
                -P "$DB_PORT" \
                -u "$DATABASE_USERNAME" \
                -p"$DATABASE_PASSWORD" \
                < /sql/init_customer_db.sql
      volumes:
        - name: sql-files
          configMap:
            name: bookstore-db-sql
YAML

echo "Created database init job bookstore-db-init in namespace $NAMESPACE"
echo "Watch it with: kubectl -n $NAMESPACE logs -f job/bookstore-db-init"
