# PowerShell script om alle personen opnieuw te matchen met de nieuwe GID logica
# Dit script roept het /api/persons/match-gids endpoint aan

$apiUrl = "http://localhost:8080/api/persons/match-gids"

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  GID Re-Matching voor Alle Personen" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Dit script zal alle personen opnieuw matchen met de nieuwe GID logica:" -ForegroundColor Yellow
Write-Host "  - Matched (100): Personen met echte IDs (niet AUTO_)" -ForegroundColor Green
Write-Host "  - Pending (30-99): Personen met AUTO_ ID maar wel matching info" -ForegroundColor Yellow
Write-Host "  - Unmatched (<30): Personen zonder bruikbare matching info" -ForegroundColor Red
Write-Host ""
Write-Host "Endpoint: $apiUrl" -ForegroundColor Gray
Write-Host ""

# Controleer of de server draait
Write-Host "Controleren of backend server draait..." -ForegroundColor Cyan
try {
    $healthCheck = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method Get -TimeoutSec 5
    Write-Host "✓ Backend server is actief" -ForegroundColor Green
    Write-Host ""
} catch {
    Write-Host "✗ Backend server is niet bereikbaar!" -ForegroundColor Red
    Write-Host "  Start de server met: cd backend && cargo run --release" -ForegroundColor Yellow
    exit 1
}

# Vraag bevestiging
Write-Host "Weet je zeker dat je ALLE personen opnieuw wilt matchen?" -ForegroundColor Yellow
Write-Host "Dit kan enkele minuten duren voor grote datasets..." -ForegroundColor Gray
$confirmation = Read-Host "Typ 'ja' om door te gaan"

if ($confirmation -ne "ja") {
    Write-Host "Operatie geannuleerd." -ForegroundColor Red
    exit 0
}

Write-Host ""
Write-Host "Starting GID re-matching..." -ForegroundColor Cyan
Write-Host ""

# Meet de uitvoeringstijd
$startTime = Get-Date

try {
    # Roep het match-gids endpoint aan
    $response = Invoke-RestMethod -Uri $apiUrl -Method Post -TimeoutSec 600

    $endTime = Get-Date
    $duration = $endTime - $startTime

    Write-Host ""
    Write-Host "================================================" -ForegroundColor Green
    Write-Host "  GID Re-Matching Voltooid!" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Resultaten:" -ForegroundColor Cyan
    Write-Host "  - Totaal verwerkt: $($response.total_processed) personen" -ForegroundColor White
    Write-Host "  - Matched: $($response.total_matched) personen" -ForegroundColor Green
    Write-Host "  - Match rate: $([math]::Round($response.match_rate, 2))%" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Uitvoeringstijd: $($duration.Minutes)m $($duration.Seconds)s" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Je kunt nu de frontend vernieuwen om de bijgewerkte statussen te zien." -ForegroundColor Yellow
    Write-Host ""

} catch {
    Write-Host ""
    Write-Host "✗ Fout tijdens re-matching:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red

    if ($_.ErrorDetails.Message) {
        Write-Host "Details: $($_.ErrorDetails.Message)" -ForegroundColor Red
    }

    Write-Host ""
    Write-Host "Controleer de backend logs voor meer informatie." -ForegroundColor Yellow
    exit 1
}
