#!/bin/bash
set -e

BASE_URL="http://localhost:8080"
FILE="/workspace/Excel/Operational Insights Feb 17 2026.csv"

echo "=== Step 1: Upload CSV ==="
UPLOAD_RESP=$(curl -s -X POST "$BASE_URL/api/imports/upload" \
  -F "file=@$FILE" \
  -F "user_id=test-user")
echo "$UPLOAD_RESP" | head -c 500
echo ""

UPLOAD_ID=$(echo "$UPLOAD_RESP" | grep -o '"upload_id":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "Upload ID: $UPLOAD_ID"

if [ -z "$UPLOAD_ID" ]; then
  echo "ERROR: No upload ID found"
  exit 1
fi

echo ""
echo "=== Step 2: Generate Preview ==="
PREVIEW_RESP=$(curl -s -X POST "$BASE_URL/api/imports/preview" \
  -H "Content-Type: application/json" \
  -d "{\"upload_id\":\"$UPLOAD_ID\",\"import_valid_only\":false}")
echo "$PREVIEW_RESP" | head -c 500
echo ""

PREVIEW_ID=$(echo "$PREVIEW_RESP" | grep -o '"preview_id":"[^"]*"' | head -1 | cut -d'"' -f4)
echo "Preview ID: $PREVIEW_ID"

if [ -z "$PREVIEW_ID" ]; then
  echo "ERROR: No preview ID found"
  echo "Full response:"
  echo "$PREVIEW_RESP"
  exit 1
fi

echo ""
echo "=== Step 3: Execute Import ==="
EXECUTE_RESP=$(curl -s -X POST "$BASE_URL/api/imports/execute" \
  -H "Content-Type: application/json" \
  -d "{\"preview_id\":\"$PREVIEW_ID\",\"confirmed\":true}")
echo "$EXECUTE_RESP"

echo ""
echo "=== Done ==="
