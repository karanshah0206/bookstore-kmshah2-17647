#!/usr/bin/env bash
set -euo pipefail

# Usage: ./deployment/scripts/sync_db_endpoint_from_cfn.sh <stack-name> [namespace]
STACK_NAME="${1:-}"
NAMESPACE="${2:-bookstore-ns}"

if [[ -z "$STACK_NAME" ]]; then
  echo "Usage: $0 <stack-name> [namespace]"
  exit 1
fi

WRITER_ENDPOINT=$(aws cloudformation describe-stacks \
  --stack-name "$STACK_NAME" \
  --query "Stacks[0].Outputs[?OutputKey=='AuroraWriterEndpoint'].OutputValue" \
  --output text)

AURORA_PORT=$(aws cloudformation describe-stacks \
  --stack-name "$STACK_NAME" \
  --query "Stacks[0].Outputs[?OutputKey=='AuroraPort'].OutputValue" \
  --output text)

if [[ -z "$WRITER_ENDPOINT" || "$WRITER_ENDPOINT" == "None" ]]; then
  echo "AuroraWriterEndpoint output not found in stack $STACK_NAME"
  exit 1
fi

if [[ -z "$AURORA_PORT" || "$AURORA_PORT" == "None" ]]; then
  AURORA_PORT="3306"
fi

kubectl create configmap bookstore-db-endpoints \
  --namespace "$NAMESPACE" \
  --from-literal=aurora_writer_endpoint="$WRITER_ENDPOINT:$AURORA_PORT" \
  --dry-run=client -o yaml | kubectl apply -f -

echo "Applied ConfigMap bookstore-db-endpoints in namespace $NAMESPACE with endpoint $WRITER_ENDPOINT:$AURORA_PORT"
