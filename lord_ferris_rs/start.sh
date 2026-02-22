#!/bin/sh
set -e

CREDENTIALS_FILE="/app/google-credentials.json"

if [ -z "$GOOGLE_CREDENTIALS_JSON" ]; then
  echo "Error: GOOGLE_CREDENTIALS_JSON environment variable not set."
  exit 1
fi

echo "$GOOGLE_CREDENTIALS_JSON" > "$CREDENTIALS_FILE"

export GOOGLE_APPLICATION_CREDENTIALS="$CREDENTIALS_FILE"

unset GOOGLE_CREDENTIALS_JSON

exec /app/lord_ferris_rs
