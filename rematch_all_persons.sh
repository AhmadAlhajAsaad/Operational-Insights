#!/bin/bash
# Bash script om alle personen opnieuw te matchen met de nieuwe GID logica
# Dit script roept het /api/persons/match-gids endpoint aan

API_URL="http://localhost:8080/api/persons/match-gids"

echo "================================================"
echo "  GID Re-Matching voor Alle Personen"
echo "================================================"
echo ""
echo -e "\033[1;33mDit script zal alle personen opnieuw matchen met de nieuwe GID logica:\033[0m"
echo -e "\033[1;32m  - Matched (100): Personen met echte IDs (niet AUTO_)\033[0m"
echo -e "\033[1;33m  - Pending (30-99): Personen met AUTO_ ID maar wel matching info\033[0m"
echo -e "\033[1;31m  - Unmatched (<30): Personen zonder bruikbare matching info\033[0m"
echo ""
echo "Endpoint: $API_URL"
echo ""

# Controleer of de server draait
echo "Controleren of backend server draait..."
if curl -s -f "http://localhost:8080/health" > /dev/null 2>&1; then
    echo -e "\033[1;32m✓ Backend server is actief\033[0m"
    echo ""
else
    echo -e "\033[1;31m✗ Backend server is niet bereikbaar!\033[0m"
    echo -e "\033[1;33m  Start de server met: cd backend && cargo run --release\033[0m"
    exit 1
fi

# Vraag bevestiging
echo -e "\033[1;33mWeet je zeker dat je ALLE personen opnieuw wilt matchen?\033[0m"
echo "Dit kan enkele minuten duren voor grote datasets..."
read -p "Typ 'ja' om door te gaan: " confirmation

if [ "$confirmation" != "ja" ]; then
    echo -e "\033[1;31mOperatie geannuleerd.\033[0m"
    exit 0
fi

echo ""
echo "Starting GID re-matching..."
echo ""

# Meet de uitvoeringstijd
START_TIME=$(date +%s)

# Roep het match-gids endpoint aan
RESPONSE=$(curl -s -X POST "$API_URL" -H "Content-Type: application/json" -w "\n%{http_code}")
HTTP_CODE=$(echo "$RESPONSE" | tail -n 1)
BODY=$(echo "$RESPONSE" | sed '$d')

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

if [ "$HTTP_CODE" -eq 200 ]; then
    echo ""
    echo "================================================"
    echo "  GID Re-Matching Voltooid!"
    echo "================================================"
    echo ""
    echo "Resultaten:"

    # Parse JSON response (requires jq, maar fallback naar raw output)
    if command -v jq &> /dev/null; then
        TOTAL_PROCESSED=$(echo "$BODY" | jq -r '.total_processed')
        TOTAL_MATCHED=$(echo "$BODY" | jq -r '.total_matched')
        MATCH_RATE=$(echo "$BODY" | jq -r '.match_rate')

        echo "  - Totaal verwerkt: $TOTAL_PROCESSED personen"
        echo -e "\033[1;32m  - Matched: $TOTAL_MATCHED personen\033[0m"
        echo -e "\033[1;36m  - Match rate: $(printf "%.2f" $MATCH_RATE)%\033[0m"
    else
        echo "$BODY"
    fi

    echo ""
    echo "Uitvoeringstijd: ${MINUTES}m ${SECONDS}s"
    echo ""
    echo -e "\033[1;33mJe kunt nu de frontend vernieuwen om de bijgewerkte statussen te zien.\033[0m"
    echo ""
else
    echo ""
    echo -e "\033[1;31m✗ Fout tijdens re-matching (HTTP $HTTP_CODE):\033[0m"
    echo "$BODY"
    echo ""
    echo -e "\033[1;33mControleer de backend logs voor meer informatie.\033[0m"
    exit 1
fi
